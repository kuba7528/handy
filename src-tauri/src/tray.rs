use crate::managers::history::{HistoryEntry, HistoryManager};
use crate::managers::model::ModelManager;
use crate::managers::transcription::TranscriptionManager;
use crate::settings;
use crate::tray_i18n::get_tray_translations;
use log::{error, info, warn};
use std::sync::{Mutex, OnceLock};
use std::sync::Arc;
use tauri::image::Image;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager, Theme};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Clone, Debug, PartialEq)]
pub enum TrayIconState {
    Idle,
    Recording,
    Transcribing,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppTheme {
    Dark,
    Light,
    /// Pink branded icons (`handy.png`, `recording.png`, `transcribing.png`) matching the UI logo.
    Colored,
}

static LAST_TRAY_ICON_STATE: OnceLock<Mutex<TrayIconState>> = OnceLock::new();

fn last_tray_icon_state() -> &'static Mutex<TrayIconState> {
    LAST_TRAY_ICON_STATE.get_or_init(|| Mutex::new(TrayIconState::Idle))
}

fn theme_from_window(app: &AppHandle) -> AppTheme {
    if let Some(main_window) = app.get_webview_window("main") {
        match main_window.theme().unwrap_or(Theme::Dark) {
            Theme::Light => AppTheme::Light,
            Theme::Dark => AppTheme::Dark,
            _ => AppTheme::Dark,
        }
    } else {
        AppTheme::Dark
    }
}

/// Tray icons follow the **application** look, not the Windows taskbar light/dark mode.
///
/// Windows and Linux use the pink branded assets so the tray matches `logo-primary` in the UI.
/// macOS keeps monochrome template icons that adapt to the menu bar via `set_icon_as_template`.
pub fn get_current_theme(app: &AppHandle) -> AppTheme {
    if cfg!(target_os = "macos") {
        theme_from_window(app)
    } else {
        AppTheme::Colored
    }
}

fn load_icon_image(app: &AppHandle, icon_path: &str) -> Option<Image<'static>> {
    let path = app
        .path()
        .resolve(icon_path, tauri::path::BaseDirectory::Resource)
        .ok()?;
    Image::from_path(path).ok()
}

fn set_tray_icon_image(app: &AppHandle, state: &TrayIconState) {
    let tray = app.state::<TrayIcon>();
    let theme = get_current_theme(app);
    let icon_path = get_icon_path(theme, state.clone());

    if let Some(icon) = load_icon_image(app, icon_path) {
        let _ = tray.set_icon(Some(icon));
    }
    apply_tray_icon_template(&tray);
}

/// Windows/Linux taskbar and title bar use the main window icon; keep it in sync with tray branding.
fn set_main_window_icon(app: &AppHandle, state: &TrayIconState) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let theme = get_current_theme(app);
    let icon_path = get_icon_path(theme, state.clone());
    if let Some(icon) = load_icon_image(app, icon_path) {
        let _ = window.set_icon(icon);
    }
}

/// Apply the current tray-state icon to the main window (taskbar / title bar on Windows).
pub fn sync_main_window_icon(app: &AppHandle) {
    let state = last_tray_icon_state()
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or(TrayIconState::Idle);
    set_main_window_icon(app, &state);
}

/// Re-apply tray + window icons for the last known state.
///
/// Custom `appearance_accent_color` does not regenerate PNG/ICO assets; icons use default Handy pink.
pub fn refresh_tray_theme(app: &AppHandle) {
    let state = last_tray_icon_state()
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or(TrayIconState::Idle);
    set_tray_icon_image(app, &state);
    set_main_window_icon(app, &state);
}

fn apply_tray_icon_template(tray: &TrayIcon) {
    #[cfg(target_os = "macos")]
    {
        let _ = tray.set_icon_as_template(true);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = tray.set_icon_as_template(false);
    }
}

/// Icon path for tray state and theme.
///
/// `AppTheme::Colored` — pink Handy branding (Windows/Linux, matches UI accent).
/// `AppTheme::Dark` / `Light` — monochrome glyphs for macOS menu bar template mode.
pub fn get_icon_path(theme: AppTheme, state: TrayIconState) -> &'static str {
    match (theme, state) {
        (AppTheme::Dark, TrayIconState::Idle) => "resources/tray_idle.png",
        (AppTheme::Dark, TrayIconState::Recording) => "resources/tray_recording.png",
        (AppTheme::Dark, TrayIconState::Transcribing) => "resources/tray_transcribing.png",
        (AppTheme::Light, TrayIconState::Idle) => "resources/tray_idle_dark.png",
        (AppTheme::Light, TrayIconState::Recording) => "resources/tray_recording_dark.png",
        (AppTheme::Light, TrayIconState::Transcribing) => "resources/tray_transcribing_dark.png",
        (AppTheme::Colored, TrayIconState::Idle) => "resources/handy.png",
        (AppTheme::Colored, TrayIconState::Recording) => "resources/recording.png",
        (AppTheme::Colored, TrayIconState::Transcribing) => "resources/transcribing.png",
    }
}

pub fn change_tray_icon(app: &AppHandle, icon: TrayIconState) {
    if let Ok(mut guard) = last_tray_icon_state().lock() {
        *guard = icon.clone();
    }
    set_tray_icon_image(app, &icon);
    set_main_window_icon(app, &icon);
    update_tray_menu(app, &icon, None);
}

pub fn tray_tooltip() -> String {
    version_label()
}

fn version_label() -> String {
    if cfg!(debug_assertions) {
        format!("Handy v{} (Dev)", env!("CARGO_PKG_VERSION"))
    } else {
        format!("Handy v{}", env!("CARGO_PKG_VERSION"))
    }
}

pub fn update_tray_menu(app: &AppHandle, state: &TrayIconState, locale: Option<&str>) {
    let settings = settings::get_settings(app);

    let locale = locale.unwrap_or(&settings.app_language);
    let strings = get_tray_translations(Some(locale.to_string()));

    // Platform-specific accelerators
    #[cfg(target_os = "macos")]
    let (settings_accelerator, quit_accelerator) = (Some("Cmd+,"), Some("Cmd+Q"));
    #[cfg(not(target_os = "macos"))]
    let (settings_accelerator, quit_accelerator) = (Some("Ctrl+,"), Some("Ctrl+Q"));

    // Create common menu items
    let version_label = version_label();
    let version_i = MenuItem::with_id(app, "version", &version_label, false, None::<&str>)
        .expect("failed to create version item");
    let settings_i = MenuItem::with_id(
        app,
        "settings",
        &strings.settings,
        true,
        settings_accelerator,
    )
    .expect("failed to create settings item");
    let check_updates_i = MenuItem::with_id(
        app,
        "check_updates",
        &strings.check_updates,
        settings.update_checks_enabled,
        None::<&str>,
    )
    .expect("failed to create check updates item");
    let copy_last_transcript_i = MenuItem::with_id(
        app,
        "copy_last_transcript",
        &strings.copy_last_transcript,
        true,
        None::<&str>,
    )
    .expect("failed to create copy last transcript item");
    let model_loaded = app.state::<Arc<TranscriptionManager>>().is_model_loaded();
    let quit_i = MenuItem::with_id(app, "quit", &strings.quit, true, quit_accelerator)
        .expect("failed to create quit item");
    let separator = || PredefinedMenuItem::separator(app).expect("failed to create separator");

    // Build model submenu — label is the active model name
    let model_manager = app.state::<Arc<ModelManager>>();
    let models = model_manager.get_available_models();
    let current_model_id = &settings.selected_model;

    let mut downloaded: Vec<_> = models.into_iter().filter(|m| m.is_downloaded).collect();
    downloaded.sort_by(|a, b| a.name.cmp(&b.name));

    let submenu_label = downloaded
        .iter()
        .find(|m| m.id == *current_model_id)
        .map(|m| m.name.clone())
        .unwrap_or_else(|| strings.model.clone());

    let model_submenu = {
        let submenu = Submenu::with_id(app, "model_submenu", &submenu_label, true)
            .expect("failed to create model submenu");

        for model in &downloaded {
            let is_active = model.id == *current_model_id;
            let item_id = format!("model_select:{}", model.id);
            let item =
                CheckMenuItem::with_id(app, &item_id, &model.name, true, is_active, None::<&str>)
                    .expect("failed to create model item");
            let _ = submenu.append(&item);
        }

        submenu
    };

    let unload_model_i = MenuItem::with_id(
        app,
        "unload_model",
        &strings.unload_model,
        model_loaded,
        None::<&str>,
    )
    .expect("failed to create unload model item");

    let menu = match state {
        TrayIconState::Recording | TrayIconState::Transcribing => {
            let cancel_i = MenuItem::with_id(app, "cancel", &strings.cancel, true, None::<&str>)
                .expect("failed to create cancel item");
            Menu::with_items(
                app,
                &[
                    &version_i,
                    &separator(),
                    &cancel_i,
                    &separator(),
                    &copy_last_transcript_i,
                    &separator(),
                    &settings_i,
                    &check_updates_i,
                    &separator(),
                    &quit_i,
                ],
            )
            .expect("failed to create menu")
        }
        TrayIconState::Idle => Menu::with_items(
            app,
            &[
                &version_i,
                &separator(),
                &copy_last_transcript_i,
                &separator(),
                &model_submenu,
                &unload_model_i,
                &separator(),
                &settings_i,
                &check_updates_i,
                &separator(),
                &quit_i,
            ],
        )
        .expect("failed to create menu"),
    };

    let tray = app.state::<TrayIcon>();
    let _ = tray.set_menu(Some(menu));
    apply_tray_icon_template(&tray);
    let _ = tray.set_tooltip(Some(version_label));
}

fn last_transcript_text(entry: &HistoryEntry) -> &str {
    entry
        .post_processed_text
        .as_deref()
        .unwrap_or(&entry.transcription_text)
}

pub fn set_tray_visibility(app: &AppHandle, visible: bool) {
    let tray = app.state::<TrayIcon>();
    if let Err(e) = tray.set_visible(visible) {
        error!("Failed to set tray visibility: {}", e);
    } else {
        info!("Tray visibility set to: {}", visible);
    }
}

pub fn copy_last_transcript(app: &AppHandle) {
    let history_manager = app.state::<Arc<HistoryManager>>();
    let entry = match history_manager.get_latest_completed_entry() {
        Ok(Some(entry)) => entry,
        Ok(None) => {
            warn!("No completed transcription history entries available for tray copy.");
            return;
        }
        Err(err) => {
            error!(
                "Failed to fetch last completed transcription entry: {}",
                err
            );
            return;
        }
    };

    let text = last_transcript_text(&entry);
    if text.trim().is_empty() {
        warn!("Last completed transcription is empty; skipping tray copy.");
        return;
    }

    if let Err(err) = app.clipboard().write_text(text) {
        error!("Failed to copy last transcript to clipboard: {}", err);
        return;
    }

    info!("Copied last transcript to clipboard via tray.");
}

#[cfg(test)]
mod tests {
    use super::{get_icon_path, AppTheme, TrayIconState};
    use super::last_transcript_text;
    use crate::managers::history::HistoryEntry;

    #[test]
    fn colored_theme_uses_branded_idle_icon() {
        assert_eq!(
            get_icon_path(AppTheme::Colored, TrayIconState::Idle),
            "resources/handy.png"
        );
    }

    #[test]
    fn colored_recording_and_transcribing_icons() {
        assert_eq!(
            get_icon_path(AppTheme::Colored, TrayIconState::Recording),
            "resources/recording.png"
        );
        assert_eq!(
            get_icon_path(AppTheme::Colored, TrayIconState::Transcribing),
            "resources/transcribing.png"
        );
    }

    #[test]
    fn macos_dark_theme_uses_light_glyph_idle_icon() {
        assert_eq!(
            get_icon_path(AppTheme::Dark, TrayIconState::Idle),
            "resources/tray_idle.png"
        );
    }

    #[test]
    fn macos_light_theme_uses_dark_glyph_idle_icon() {
        assert_eq!(
            get_icon_path(AppTheme::Light, TrayIconState::Idle),
            "resources/tray_idle_dark.png"
        );
    }

    fn build_entry(transcription: &str, post_processed: Option<&str>) -> HistoryEntry {
        HistoryEntry {
            id: 1,
            file_name: "handy-1.wav".to_string(),
            timestamp: 0,
            saved: false,
            title: "Recording".to_string(),
            transcription_text: transcription.to_string(),
            post_processed_text: post_processed.map(|text| text.to_string()),
            post_process_prompt: None,
            post_process_requested: false,
        }
    }

    #[test]
    fn uses_post_processed_text_when_available() {
        let entry = build_entry("raw", Some("processed"));
        assert_eq!(last_transcript_text(&entry), "processed");
    }

    #[test]
    fn falls_back_to_raw_transcription() {
        let entry = build_entry("raw", None);
        assert_eq!(last_transcript_text(&entry), "raw");
    }
}
