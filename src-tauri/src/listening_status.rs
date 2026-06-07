use crate::managers::audio::AudioRecordingManager;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager};

const LEVEL_EMIT_INTERVAL_MS: u64 = 33;
static LAST_LEVEL_EMIT_MS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListeningStatus {
    Idle,
    Listening,
    Recording,
    Transcribing,
    Processing,
}

impl ListeningStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Listening => "listening",
            Self::Recording => "recording",
            Self::Transcribing => "transcribing",
            Self::Processing => "processing",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "listening" => Self::Listening,
            "recording" => Self::Recording,
            "transcribing" => Self::Transcribing,
            "processing" => Self::Processing,
            _ => Self::Idle,
        }
    }
}

pub struct ListeningStatusStore(Mutex<ListeningStatus>);

impl Default for ListeningStatusStore {
    fn default() -> Self {
        Self(Mutex::new(ListeningStatus::Idle))
    }
}

pub fn set_listening_status(app_handle: &AppHandle, status: ListeningStatus) {
    if let Some(store) = app_handle.try_state::<ListeningStatusStore>() {
        *store.0.lock().unwrap() = status;
    }
    emit_listening_event(app_handle, "listening-status", status.as_str());

    if status == ListeningStatus::Idle && crate::listening_compact::is_compact_mode() {
        let _ = crate::listening_compact::exit_listening_compact_mode(app_handle);
    }
}

pub fn get_listening_status(app_handle: &AppHandle) -> ListeningStatus {
    app_handle
        .try_state::<ListeningStatusStore>()
        .map(|store| *store.0.lock().unwrap())
        .unwrap_or(ListeningStatus::Idle)
}

pub fn show_listening_indicator(app_handle: &AppHandle) {
    set_listening_status(app_handle, ListeningStatus::Listening);
}

pub fn show_recording_overlay(app_handle: &AppHandle) {
    set_listening_status(app_handle, ListeningStatus::Recording);
}

pub fn show_transcribing_overlay(app_handle: &AppHandle) {
    set_listening_status(app_handle, ListeningStatus::Transcribing);
}

pub fn show_processing_overlay(app_handle: &AppHandle) {
    set_listening_status(app_handle, ListeningStatus::Processing);
}

pub fn hide_recording_overlay(app_handle: &AppHandle) {
    let continuous_active = app_handle
        .try_state::<Arc<AudioRecordingManager>>()
        .map(|rm| rm.is_continuous())
        .unwrap_or(false);

    if continuous_active {
        set_listening_status(app_handle, ListeningStatus::Listening);
    } else {
        set_listening_status(app_handle, ListeningStatus::Idle);
    }
}

pub fn emit_levels(app_handle: &AppHandle, levels: &Vec<f32>) {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let last = LAST_LEVEL_EMIT_MS.load(Ordering::Relaxed);
    if now_ms.saturating_sub(last) < LEVEL_EMIT_INTERVAL_MS {
        return;
    }
    LAST_LEVEL_EMIT_MS.store(now_ms, Ordering::Relaxed);

    emit_listening_event(app_handle, "mic-level", levels);
}

fn emit_listening_event<T: Clone + serde::Serialize>(
    app_handle: &AppHandle,
    event: &str,
    payload: T,
) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit(event, payload.clone());
    }
    if let Some(pill) =
        app_handle.get_webview_window(crate::listening_compact::LISTENING_PILL_LABEL)
    {
        let _ = pill.emit(event, payload);
    }
}
