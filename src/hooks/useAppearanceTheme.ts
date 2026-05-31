import { useEffect } from "react";
import type { AppSettings } from "@/bindings";
import {
  CONTROL_SCALE_VALUES,
  FONT_SCALE_VALUES,
  accentToUiColor,
} from "@/lib/constants/appearance";

const COLOR_VARS = [
  "--color-logo-primary",
  "--color-background-ui",
  "--color-background",
  "--color-text",
] as const;

function clearCustomColors(root: HTMLElement) {
  for (const varName of COLOR_VARS) {
    root.style.removeProperty(varName);
  }
}

function applyColorScheme(root: HTMLElement, scheme: AppSettings["appearance_color_scheme"]) {
  if (scheme === "light" || scheme === "dark") {
    root.dataset.colorScheme = scheme;
  } else {
    delete root.dataset.colorScheme;
  }
}

function applyAppearanceSettings(settings: AppSettings) {
  const root = document.documentElement;

  clearCustomColors(root);
  applyColorScheme(root, settings.appearance_color_scheme ?? "auto");

  const accent = settings.appearance_accent_color?.trim();
  if (accent) {
    root.style.setProperty("--color-logo-primary", accent);
    root.style.setProperty("--color-background-ui", accentToUiColor(accent));
  }

  const background = settings.appearance_background_color?.trim();
  if (background) {
    root.style.setProperty("--color-background", background);
  }

  const text = settings.appearance_text_color?.trim();
  if (text) {
    root.style.setProperty("--color-text", text);
  }

  const fontScale =
    FONT_SCALE_VALUES[settings.appearance_font_scale ?? "medium"] ?? 1;
  root.style.setProperty("--font-scale", String(fontScale));

  const controlScale =
    CONTROL_SCALE_VALUES[settings.appearance_control_density ?? "normal"] ?? 1;
  root.style.setProperty("--control-scale", String(controlScale));
}

export function useAppearanceTheme(settings: AppSettings | null) {
  useEffect(() => {
    if (!settings) return;

    applyAppearanceSettings(settings);

    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const handleSchemeChange = () => {
      if ((settings.appearance_color_scheme ?? "auto") === "auto") {
        applyAppearanceSettings(settings);
      }
    };

    media.addEventListener("change", handleSchemeChange);
    return () => media.removeEventListener("change", handleSchemeChange);
  }, [settings]);
}
