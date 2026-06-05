use tauri::image::Image;

/// Default Handy pink in branded PNG assets (`#faa2ca`).
pub const DEFAULT_ACCENT: (u8, u8, u8) = (250, 162, 202);

fn relative_luminance(r: u8, g: u8, b: u8) -> f32 {
    0.2126 * f32::from(r) + 0.7152 * f32::from(g) + 0.0722 * f32::from(b)
}

/// Parse `#rgb` or `#rrggbb` into RGB bytes.
pub fn parse_hex_color(value: &str) -> Option<(u8, u8, u8)> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            Some((r * 17, g * 17, b * 17))
        }
        _ => None,
    }
}

pub fn needs_tint(target: (u8, u8, u8)) -> bool {
    target != DEFAULT_ACCENT
}

/// Recolor pink branded pixels to `target`, preserving per-pixel luminance ratios.
pub fn recolor_rgba(rgba: &mut [u8], target: (u8, u8, u8)) {
    if target == DEFAULT_ACCENT {
        return;
    }
    let (tr, tg, tb) = target;
    let src_lum = relative_luminance(DEFAULT_ACCENT.0, DEFAULT_ACCENT.1, DEFAULT_ACCENT.2);
    if src_lum <= f32::EPSILON {
        return;
    }

    for chunk in rgba.chunks_exact_mut(4) {
        let alpha = chunk[3];
        if alpha < 10 {
            continue;
        }
        let ratio = relative_luminance(chunk[0], chunk[1], chunk[2]) / src_lum;
        chunk[0] = (f32::from(tr) * ratio).clamp(0.0, 255.0) as u8;
        chunk[1] = (f32::from(tg) * ratio).clamp(0.0, 255.0) as u8;
        chunk[2] = (f32::from(tb) * ratio).clamp(0.0, 255.0) as u8;
    }
}

pub fn tint_image(img: Image<'_>, target: (u8, u8, u8)) -> Image<'static> {
    let mut rgba = img.rgba().to_vec();
    recolor_rgba(&mut rgba, target);
    Image::new_owned(rgba, img.width(), img.height())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_six_digit_hex() {
        assert_eq!(parse_hex_color("#3b82f6"), Some((59, 130, 246)));
        assert_eq!(parse_hex_color("3b82f6"), Some((59, 130, 246)));
    }

    #[test]
    fn parse_three_digit_hex() {
        assert_eq!(parse_hex_color("#f0a"), Some((255, 0, 170)));
    }

    #[test]
    fn parse_rejects_invalid() {
        assert_eq!(parse_hex_color(""), None);
        assert_eq!(parse_hex_color("not-a-color"), None);
    }

    #[test]
    fn default_accent_does_not_need_tint() {
        assert!(!needs_tint(DEFAULT_ACCENT));
        assert!(needs_tint((59, 130, 246)));
    }

    #[test]
    fn recolor_maps_source_accent_to_target() {
        let mut rgba = vec![250, 162, 202, 255];
        recolor_rgba(&mut rgba, (59, 130, 246));
        assert_ne!((rgba[0], rgba[1], rgba[2]), DEFAULT_ACCENT);
        assert_eq!(rgba[3], 255);
    }

    #[test]
    fn recolor_preserves_transparent_pixels() {
        let mut rgba = vec![0, 0, 0, 0, 250, 162, 202, 128];
        recolor_rgba(&mut rgba, (59, 130, 246));
        assert_eq!(&rgba[0..4], &[0, 0, 0, 0]);
        assert_ne!(rgba[4], 250);
    }

    #[test]
    fn recolor_default_target_is_unchanged() {
        let mut rgba = vec![250, 162, 202, 255, 249, 161, 201, 200];
        let expected = rgba.clone();
        recolor_rgba(&mut rgba, DEFAULT_ACCENT);
        assert_eq!(rgba, expected);
    }
}
