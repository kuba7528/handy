use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder};

pub const LISTENING_PILL_LABEL: &str = "listening_pill";

static COMPACT_MODE: AtomicBool = AtomicBool::new(false);

fn emit_compact_mode_error(app: &AppHandle, message: impl Into<String>) -> String {
    let message = message.into();
    log::error!("Listening compact mode error: {message}");
    let _ = app.emit("listening-compact-mode-error", message.clone());
    message
}

pub fn is_compact_mode() -> bool {
    COMPACT_MODE.load(Ordering::Relaxed)
}

pub fn enter_listening_compact_mode(app: &AppHandle) -> Result<(), String> {
    if is_compact_mode() {
        return Ok(());
    }

    ensure_pill_window(app).map_err(|e| emit_compact_mode_error(app, e))?;
    if let Some(pill) = app.get_webview_window(LISTENING_PILL_LABEL) {
        pill.show().map_err(|e| {
            emit_compact_mode_error(app, format!("Failed to show pill window: {e}"))
        })?;
        if let Err(e) = center_window(&pill) {
            log::warn!("Failed to center listening pill window: {e}");
        }
        pill.set_focus().map_err(|e| {
            emit_compact_mode_error(app, format!("Failed to focus pill window: {e}"))
        })?;
    } else {
        return Err(emit_compact_mode_error(
            app,
            "Listening pill window was not created",
        ));
    }

    if let Some(main) = app.get_webview_window("main") {
        main.hide().map_err(|e| {
            emit_compact_mode_error(app, format!("Failed to hide main window: {e}"))
        })?;
    }

    COMPACT_MODE.store(true, Ordering::Relaxed);
    let _ = app.emit("listening-compact-mode", true);
    log::info!("Entered listening compact mode");
    Ok(())
}

pub fn exit_listening_compact_mode(app: &AppHandle) -> Result<(), String> {
    if !is_compact_mode() {
        return Ok(());
    }

    COMPACT_MODE.store(false, Ordering::Relaxed);

    if let Some(pill) = app.get_webview_window(LISTENING_PILL_LABEL) {
        let _ = pill.hide();
    }

    crate::show_main_window(app);
    let _ = app.emit("listening-compact-mode", false);
    Ok(())
}

fn ensure_pill_window(app: &AppHandle) -> Result<(), String> {
    if app.get_webview_window(LISTENING_PILL_LABEL).is_some() {
        return Ok(());
    }

    let mut builder = WebviewWindowBuilder::new(
        app,
        LISTENING_PILL_LABEL,
        WebviewUrl::App("/listening-pill.html".into()),
    )
    .title("Handy")
    .inner_size(320.0, 56.0)
    .min_inner_size(320.0, 56.0)
    .resizable(false)
    .maximizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .visible(false)
    .transparent(true);

    if let Some(data_dir) = crate::portable::data_dir() {
        builder = builder.data_directory(data_dir.join("webview-pill"));
    }

    builder.build().map_err(|e| {
        emit_compact_mode_error(app, format!("Failed to create pill window: {e}"))
    })?;
    Ok(())
}

fn center_window(window: &tauri::WebviewWindow) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .or_else(|| window.primary_monitor().ok().flatten())
        .ok_or_else(|| "No monitor found".to_string())?;

    let monitor_size = monitor.size();
    let monitor_pos = monitor.position();
    let window_size = window
        .outer_size()
        .or_else(|_| window.inner_size())
        .map_err(|e| e.to_string())?;

    let x = monitor_pos.x + (monitor_size.width as i32 - window_size.width as i32) / 2;
    let y = monitor_pos.y + (monitor_size.height as i32 - window_size.height as i32) / 2;

    window
        .set_position(PhysicalPosition::new(x, y))
        .map_err(|e| e.to_string())?;
    Ok(())
}
