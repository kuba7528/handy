#!/usr/bin/env python3
"""Regenerate Tauri bundle icons from pink Handy branding (logo / handy tray asset)."""

from __future__ import annotations

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ICONS = ROOT / "src-tauri" / "icons"
# Full-resolution pink logo (matches UI); tray uses resources/handy.png at runtime.
SOURCE = ICONS / "logo.png"
FALLBACK = ROOT / "src-tauri" / "resources" / "handy.png"


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
    src = load_source()
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
    print(f"Wrote bundle icons under {ICONS}")


if __name__ == "__main__":
    main()
