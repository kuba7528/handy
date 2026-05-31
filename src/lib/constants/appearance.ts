import type {
  ColorScheme,
  ControlDensity,
  FontSizeScale,
} from "@/bindings";

export const FONT_SCALE_VALUES: Record<FontSizeScale, number> = {
  small: 0.9,
  medium: 1,
  large: 1.15,
};

export const CONTROL_SCALE_VALUES: Record<ControlDensity, number> = {
  compact: 0.85,
  normal: 1,
  comfortable: 1.15,
};

export const ACCENT_COLOR_PRESETS = [
  "#faa2ca",
  "#da5893",
  "#6366f1",
  "#3b82f6",
  "#14b8a6",
  "#22c55e",
  "#f97316",
  "#ef4444",
];

export const BACKGROUND_COLOR_PRESETS = [
  "#fbfbfb",
  "#ffffff",
  "#f3f4f6",
  "#2c2b29",
  "#1e1e1e",
  "#0f172a",
];

export const TEXT_COLOR_PRESETS = [
  "#0f0f0f",
  "#374151",
  "#fbfbfb",
  "#e5e7eb",
  "#ffffff",
];

export function isValidHexColor(value: string): boolean {
  return /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/.test(value.trim());
}

export function normalizeHexColor(value: string): string | null {
  const trimmed = value.trim();
  if (!trimmed) return null;
  const withHash = trimmed.startsWith("#") ? trimmed : `#${trimmed}`;
  if (!isValidHexColor(withHash)) return null;
  if (withHash.length === 4) {
    const [, r, g, b] = withHash;
    return `#${r}${r}${g}${g}${b}${b}`.toLowerCase();
  }
  return withHash.toLowerCase();
}

export function accentToUiColor(accent: string): string {
  return accent;
}
