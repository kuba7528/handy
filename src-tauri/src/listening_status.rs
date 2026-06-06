use crate::managers::audio::AudioRecordingManager;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};

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
    let _ = app_handle.emit("listening-status", status.as_str());
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
    let _ = app_handle.emit("mic-level", levels);
}
