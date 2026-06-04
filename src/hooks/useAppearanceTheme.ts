import { useEffect } from "react";
import type { AppSettings, ColorScheme } from "@/bindings";
import {
  CONTROL_SCALE_VALUES,
  FONT_SCALE_VALUES,
  accentToUiColor,
  paletteForScheme,
  resolveEffectiveColorScheme,
} from "@/lib/constants/appearance";

const THEME_COLOR_VARS = [
  "--color-text",
  "--color-background",
  "--color-logo-primary",
  "--color-logo-stroke",
  "--color-background-ui",
] as const;

function clearThemeColorVars(root: HTMLElement) {
  for (const varName of THEME_COLOR_VARS) {
    root.style.removeProperty(varName);
  }
}

function applyColorSchemeAttribute(
  root: HTMLElement,
  scheme: ColorScheme | undefined,
) {
  if (scheme === "light" || scheme === "dark") {
    root.setAttribute("data-color-scheme", scheme);
  } else {
    root.removeAttribute("data-color-scheme");
  }
}

function applyPalette(root: HTMLElement, palette: ReturnType<typeof paletteForScheme>) {
  root.style.setProperty("--color-text", palette.text);
  root.style.setProperty("--color-background", palette.background);
  root.style.setProperty("--color-logo-primary", palette.logoPrimary);
  root.style.setProperty("--color-logo-stroke", palette.logoStroke);
  root.style.setProperty("--color-background-ui", palette.backgroundUi);
}

export function applyAppearanceSettings(settings: AppSettings) {
  const root = document.documentElement;
  const scheme = settings.appearance_color_scheme ?? "auto";
  const effective = resolveEffectiveColorScheme(scheme);

  clearThemeColorVars(root);
  applyColorSchemeAttribute(root, scheme);
  root.style.colorScheme = effective;

  const palette = paletteForScheme(effective);
  applyPalette(root, palette);

  const accent = settings.appearance_accent_color?.trim();
  if (accent) {
    root.style.setProperty("--color-logo-primary", accent);
    root.style.setProperty("--color-background-ui", accentToUiColor(accent));
  }

  // Custom background/text only apply in auto — forced light/dark use the palette above.
  if (scheme === "auto") {
    const background = settings.appearance_background_color?.trim();
    if (background) {
      root.style.setProperty("--color-background", background);
    }

    const text = settings.appearance_text_color?.trim();
    if (text) {
      root.style.setProperty("--color-text", text);
    }
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
  }, [
    settings,
    settings?.appearance_color_scheme,
    settings?.appearance_accent_color,
    settings?.appearance_background_color,
    settings?.appearance_text_color,
    settings?.appearance_font_scale,
    settings?.appearance_control_density,
  ]);
}
