#!/usr/bin/env python3
"""Regenerate Tauri bundle icons from pink Handy branding (logo / handy tray asset)."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ICONS = ROOT / "src-tauri" / "icons"
# Full-resolution pink logo (matches UI); tray uses resources/handy.png at runtime.
SOURCE = ICONS / "logo.png"
FALLBACK = ROOT / "src-tauri" / "resources" / "handy.png"
DEFAULT_ACCENT = (250, 162, 202)


def relative_luminance(r: int, g: int, b: int) -> float:
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def parse_hex_color(value: str | None) -> tuple[int, int, int] | None:
    if not value:
        return None
    trimmed = value.strip()
    if not trimmed:
        return None
    hex_digits = trimmed[1:] if trimmed.startswith("#") else trimmed
    if len(hex_digits) == 6:
        return (
            int(hex_digits[0:2], 16),
            int(hex_digits[2:4], 16),
            int(hex_digits[4:6], 16),
        )
    if len(hex_digits) == 3:
        return (
            int(hex_digits[0], 16) * 17,
            int(hex_digits[1], 16) * 17,
            int(hex_digits[2], 16) * 17,
        )
    return None


def recolor_rgba(rgba: Image.Image, target: tuple[int, int, int]) -> Image.Image:
    if target == DEFAULT_ACCENT:
        return rgba
    tr, tg, tb = target
    src_lum = relative_luminance(*DEFAULT_ACCENT)
    if src_lum <= 1e-6:
        return rgba
    pixels = rgba.load()
    w, h = rgba.size
    for y in range(h):
        for x in range(w):
            r, g, b, a = pixels[x, y]
            if a < 10:
                continue
            ratio = relative_luminance(r, g, b) / src_lum
            pixels[x, y] = (
                min(255, int(tr * ratio)),
                min(255, int(tg * ratio)),
                min(255, int(tb * ratio)),
                a,
            )
    return rgba


def accent_from_settings_path(path: Path) -> tuple[int, int, int] | None:
    if not path.is_file():
        return None
    data = json.loads(path.read_text(encoding="utf-8"))
    settings = data.get("settings") if isinstance(data, dict) else None
    if not isinstance(settings, dict):
        return None
    raw = settings.get("appearance_accent_color")
    if not isinstance(raw, str):
        return None
    return parse_hex_color(raw)


def load_source() -> Image.Image:
    path = SOURCE if SOURCE.exists() else FALLBACK
    img = Image.open(path).convert("RGBA")
    side = max(img.size)
    canvas = Image.new("RGBA", (side, side), (0, 0, 0, 0))
    ox = (side - img.width) // 2
    oy = (side - img.height) // 2
    canvas.paste(img, (ox, oy), img)
    return canvas


def save_png(img: Image.Image, size: int, dest: Path) -> None:
    resized = img.resize((size, size), Image.Resampling.LANCZOS)
    resized.save(dest, format="PNG", optimize=True)


def save_ico(img: Image.Image, dest: Path) -> None:
    sizes = [256, 128, 64, 48, 32, 24, 16]
    frames = [img.resize((s, s), Image.Resampling.LANCZOS) for s in sizes]
    frames[0].save(
        dest,
        format="ICO",
        sizes=[(s, s) for s in sizes],
        append_images=frames[1:],
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--accent",
        help="Target accent as #rrggbb (overrides settings file)",
    )
    parser.add_argument(
        "--settings",
        type=Path,
        help="Path to settings_store.json (reads appearance_accent_color)",
    )
    args = parser.parse_args()

    accent = parse_hex_color(args.accent) if args.accent else None
    if accent is None and args.settings:
        accent = accent_from_settings_path(args.settings)
    if accent is None:
        accent = DEFAULT_ACCENT

    src = load_source()
    src = recolor_rgba(src, accent)
    ICONS.mkdir(parents=True, exist_ok=True)

    outputs = {
        "32x32.png": 32,
        "64x64.png": 64,
        "128x128.png": 128,
        "128x128@2x.png": 256,
        "icon.png": 512,
        "Square30x30Logo.png": 30,
        "Square44x44Logo.png": 44,
        "Square71x71Logo.png": 71,
        "Square89x89Logo.png": 89,
        "Square107x107Logo.png": 107,
        "Square142x142Logo.png": 142,
        "Square150x150Logo.png": 150,
        "Square284x284Logo.png": 284,
        "Square310x310Logo.png": 310,
        "StoreLogo.png": 50,
    }
    for name, size in outputs.items():
        save_png(src, size, ICONS / name)
    save_ico(src, ICONS / "icon.ico")
    accent_hex = "#{:02x}{:02x}{:02x}".format(*accent)
    print(f"Wrote bundle icons under {ICONS} (accent {accent_hex})")


if __name__ == "__main__":
    main()
