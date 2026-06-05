use crate::actions::process_transcription_output;
use crate::managers::{
    history::{HistoryEntry, HistoryManager, PaginatedHistory},
    transcription::TranscriptionManager,
};
use chrono::{DateTime, Local, TimeZone, Utc};
use std::sync::Arc;
use tauri::{AppHandle, State};

#[tauri::command]
#[specta::specta]
pub async fn get_history_entries(
    _app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    cursor: Option<i64>,
    limit: Option<usize>,
) -> Result<PaginatedHistory, String> {
    history_manager
        .get_history_entries(cursor, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn toggle_history_entry_saved(
    _app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    id: i64,
) -> Result<(), String> {
    history_manager
        .toggle_saved_status(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn get_audio_file_path(
    _app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    file_name: String,
) -> Result<String, String> {
    let path = history_manager.get_audio_file_path(&file_name);
    path.to_str()
        .ok_or_else(|| "Invalid file path".to_string())
        .map(|s| s.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_history_entry(
    _app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    id: i64,
) -> Result<(), String> {
    history_manager
        .delete_entry(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn retry_history_entry_transcription(
    app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    transcription_manager: State<'_, Arc<TranscriptionManager>>,
    id: i64,
) -> Result<(), String> {
    let entry = history_manager
        .get_entry_by_id(id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("History entry {} not found", id))?;

    let audio_path = history_manager.get_audio_file_path(&entry.file_name);
    let samples = crate::audio_toolkit::read_wav_samples(&audio_path)
        .map_err(|e| format!("Failed to load audio: {}", e))?;

    if samples.is_empty() {
        return Err("Recording has no audio samples".to_string());
    }

    transcription_manager.initiate_model_load();

    let tm = Arc::clone(&transcription_manager);
    let transcription = tauri::async_runtime::spawn_blocking(move || tm.transcribe(samples))
        .await
        .map_err(|e| format!("Transcription task panicked: {}", e))?
        .map_err(|e| e.to_string())?;

    if transcription.is_empty() {
        return Err("Recording contains no speech".to_string());
    }

    let processed =
        process_transcription_output(&app, &transcription, entry.post_process_requested).await;
    history_manager
        .update_transcription(
            id,
            transcription,
            processed.post_processed_text,
            processed.post_process_prompt,
        )
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn update_history_limit(
    app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    limit: usize,
) -> Result<(), String> {
    let mut settings = crate::settings::get_settings(&app);
    settings.history_limit = limit;
    crate::settings::write_settings(&app, settings);

    history_manager
        .cleanup_old_entries()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn update_recording_retention_period(
    app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    period: String,
) -> Result<(), String> {
    use crate::settings::RecordingRetentionPeriod;

    let retention_period = match period.as_str() {
        "never" => RecordingRetentionPeriod::Never,
        "preserve_limit" => RecordingRetentionPeriod::PreserveLimit,
        "days3" => RecordingRetentionPeriod::Days3,
        "weeks2" => RecordingRetentionPeriod::Weeks2,
        "months3" => RecordingRetentionPeriod::Months3,
        _ => return Err(format!("Invalid retention period: {}", period)),
    };

    let mut settings = crate::settings::get_settings(&app);
    settings.recording_retention_period = retention_period;
    crate::settings::write_settings(&app, settings);

    history_manager
        .cleanup_old_entries()
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn format_entry_timestamp(timestamp: i64) -> String {
    let dt: DateTime<Local> = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .map(|utc| utc.with_timezone(&Local))
        .unwrap_or_else(|| Local::now());
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn entry_display_text(entry: &HistoryEntry) -> &str {
    entry
        .post_processed_text
        .as_deref()
        .filter(|text| !text.trim().is_empty())
        .unwrap_or(&entry.transcription_text)
}

fn export_as_txt(entries: &[HistoryEntry]) -> String {
    entries
        .iter()
        .map(|entry| {
            format!(
                "[{}]\n{}\n",
                format_entry_timestamp(entry.timestamp),
                entry_display_text(entry)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn csv_escape(value: &str) -> String {
    let escaped = value.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

fn export_as_csv(entries: &[HistoryEntry]) -> String {
    let mut lines = vec!["id,timestamp,date,saved,text".to_string()];
    for entry in entries {
        lines.push(format!(
            "{},{},{},{},{}",
            entry.id,
            entry.timestamp,
            csv_escape(&format_entry_timestamp(entry.timestamp)),
            entry.saved,
            csv_escape(entry_display_text(entry)),
        ));
    }
    lines.join("\n")
}

#[tauri::command]
#[specta::specta]
pub async fn export_history(
    _app: AppHandle,
    history_manager: State<'_, Arc<HistoryManager>>,
    format: String,
) -> Result<String, String> {
    let paginated = history_manager
        .get_history_entries(None, None)
        .await
        .map_err(|e| e.to_string())?;

    match format.as_str() {
        "txt" => Ok(export_as_txt(&paginated.entries)),
        "csv" => Ok(export_as_csv(&paginated.entries)),
        other => Err(format!("Unsupported export format: {other}")),
    }
}
