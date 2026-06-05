//! Accent-tinted `.ico` for Windows shortcuts (sidecar next to `handy.exe`).
//!
//! The PE icon embedded in `handy.exe` is fixed at compile time; runtime tinting
//! only updates tray/window icons. This module writes `handy-accent.ico` beside the
//! executable so users can pin a shortcut with the current accent color.

use crate::icon_tint;
use crate::settings;
use image::imageops::FilterType;
use image::{ImageBuffer, RgbaImage};
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

const ICO_SIZES: [u32; 7] = [256, 128, 64, 48, 32, 24, 16];
pub const SIDECAR_ICO_NAME: &str = "handy-accent.ico";
const BRAND_ICON: &str = "resources/handy.png";

fn load_brand_rgba(app: &AppHandle) -> Result<(Vec<u8>, u32, u32), String> {
    let path = app
        .path()
        .resolve(BRAND_ICON, tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve brand icon: {e}"))?;
    let img = image::open(&path).map_err(|e| format!("Failed to open brand icon: {e}"))?;
    let rgba = img.to_rgba8();
    Ok((rgba.into_raw(), rgba.width(), rgba.height()))
}

fn square_canvas(rgba: Vec<u8>, width: u32, height: u32) -> RgbaImage {
    if width == height {
        return ImageBuffer::from_raw(width, height, rgba).expect("valid rgba buffer");
    }
    let side = width.max(height);
    let mut canvas: RgbaImage = ImageBuffer::new(side, side);
    let ox = (side - width) / 2;
    let oy = (side - height) / 2;
    let src: RgbaImage = ImageBuffer::from_raw(width, height, rgba).expect("valid rgba buffer");
    image::imageops::overlay(&mut canvas, &src, i64::from(ox), i64::from(oy));
    canvas
}

fn write_accent_ico(canvas: &RgbaImage, accent: (u8, u8, u8), dest: &Path) -> Result<(), String> {
    let mut dir = IconDir::new(ResourceType::Icon);
    for &size in &ICO_SIZES {
        let resized = image::imageops::resize(canvas, size, size, FilterType::Lanczos3);
        let mut rgba = resized.into_raw();
        icon_tint::recolor_rgba(&mut rgba, accent);
        let icon_image = IconImage::from_rgba_data(size, size, rgba);
        let entry = IconDirEntry::encode(&icon_image)
            .map_err(|e| format!("Failed to encode {size}px icon frame: {e}"))?;
        dir.add_entry(entry);
    }
    let file = File::create(dest).map_err(|e| format!("Failed to create {}: {e}", dest.display()))?;
    let mut writer = BufWriter::new(file);
    dir.write(&mut writer)
        .map_err(|e| format!("Failed to write ICO: {e}"))?;
    Ok(())
}

/// Directory containing `handy.exe` (portable deploy folder or install dir).
pub fn exe_directory() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| format!("Failed to locate executable: {e}"))?;
    exe.parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "Executable has no parent directory".to_string())
}

pub fn sidecar_ico_path() -> Result<PathBuf, String> {
    Ok(exe_directory()?.join(SIDECAR_ICO_NAME))
}

/// Write `handy-accent.ico` next to the running executable using current accent settings.
pub fn regenerate_sidecar_icon(app: &AppHandle) -> Result<String, String> {
    let accent = settings::get_settings(app)
        .appearance_accent_color
        .as_deref()
        .and_then(icon_tint::parse_hex_color)
        .unwrap_or(icon_tint::DEFAULT_ACCENT);

    let (rgba, width, height) = load_brand_rgba(app)?;
    let canvas = square_canvas(rgba, width, height);
    let dest = sidecar_ico_path()?;
    write_accent_ico(&canvas, accent, &dest)?;
    Ok(dest.to_string_lossy().into_owned())
}
