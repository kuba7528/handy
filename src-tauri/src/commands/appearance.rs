use crate::settings::{
    get_default_settings, get_settings, write_settings, ColorScheme, ControlDensity, FontSizeScale,
};
use crate::tray;
use tauri::AppHandle;

fn normalize_color(color: Option<String>) -> Option<String> {
    color.and_then(|c| {
        let trimmed = c.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

#[tauri::command]
#[specta::specta]
pub fn change_appearance_accent_color(
    app: AppHandle,
    color: Option<String>,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.appearance_accent_color = normalize_color(color);
    write_settings(&app, settings);
    tray::refresh_tray_theme(&app);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_appearance_background_color(
    app: AppHandle,
    color: Option<String>,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.appearance_background_color = normalize_color(color);
    write_settings(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_appearance_text_color(
    app: AppHandle,
    color: Option<String>,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.appearance_text_color = normalize_color(color);
    write_settings(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_appearance_font_scale(
    app: AppHandle,
    scale: FontSizeScale,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.appearance_font_scale = scale;
    write_settings(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_appearance_control_density(
    app: AppHandle,
    density: ControlDensity,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.appearance_control_density = density;
    write_settings(&app, settings);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn change_appearance_color_scheme(
    app: AppHandle,
    scheme: ColorScheme,
) -> Result<(), String> {
    let mut settings = get_settings(&app);
    settings.appearance_color_scheme = scheme;
    write_settings(&app, settings);
    tray::refresh_tray_theme(&app);
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn reset_appearance_settings(app: AppHandle) -> Result<(), String> {
    let defaults = get_default_settings();
    let mut settings = get_settings(&app);
    settings.appearance_accent_color = defaults.appearance_accent_color;
    settings.appearance_background_color = defaults.appearance_background_color;
    settings.appearance_text_color = defaults.appearance_text_color;
    settings.appearance_font_scale = defaults.appearance_font_scale;
    settings.appearance_control_density = defaults.appearance_control_density;
    settings.appearance_color_scheme = defaults.appearance_color_scheme;
    write_settings(&app, settings);
    tray::refresh_tray_theme(&app);
    Ok(())
}
