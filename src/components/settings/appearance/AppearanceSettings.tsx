import React, { useCallback, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { type as osType } from "@tauri-apps/plugin-os";
import type { ColorScheme, ControlDensity, FontSizeScale } from "@/bindings";
import { commands } from "@/bindings";
import { useSettings } from "../../../hooks/useSettings";
import { SettingContainer } from "../../ui/SettingContainer";
import { SettingsGroup } from "../../ui/SettingsGroup";
import { Select, type SelectOption } from "../../ui/Select";
import { Input } from "../../ui/Input";
import { Button } from "../../ui/Button";
import { ResetButton } from "../../ui/ResetButton";
import {
  ACCENT_COLOR_PRESETS,
  BACKGROUND_COLOR_PRESETS,
  TEXT_COLOR_PRESETS,
  isValidHexColor,
  normalizeHexColor,
} from "@/lib/constants/appearance";

interface ColorPickerProps {
  title: string;
  description: string;
  value: string | null | undefined;
  presets: string[];
  onChange: (color: string | null) => void;
  isUpdating: boolean;
  grouped?: boolean;
}

const ColorPicker: React.FC<ColorPickerProps> = ({
  title,
  description,
  value,
  presets,
  onChange,
  isUpdating,
  grouped = true,
}) => {
  const { t } = useTranslation();
  const [draft, setDraft] = useState(value ?? "");
  const [error, setError] = useState(false);

  React.useEffect(() => {
    setDraft(value ?? "");
    setError(false);
  }, [value]);

  const commitColor = useCallback(
    (next: string | null) => {
      onChange(next);
    },
    [onChange],
  );

  const handleBlur = () => {
    if (!draft.trim()) {
      setError(false);
      commitColor(null);
      return;
    }
    const normalized = normalizeHexColor(draft);
    if (!normalized) {
      setError(true);
      return;
    }
    setError(false);
    setDraft(normalized);
    commitColor(normalized);
  };

  return (
    <SettingContainer
      title={title}
      description={description}
      descriptionMode="tooltip"
      grouped={grouped}
      layout="stacked"
    >
      <div className="flex flex-col gap-3">
        <div className="flex flex-wrap gap-2">
          {presets.map((color) => {
            const isSelected = value?.toLowerCase() === color.toLowerCase();
            return (
              <button
                key={color}
                type="button"
                aria-label={color}
                disabled={isUpdating}
                className={`w-8 h-8 rounded-md border-2 transition-transform hover:scale-105 disabled:opacity-50 ${
                  isSelected
                    ? "border-logo-primary ring-2 ring-logo-primary/40"
                    : "border-mid-gray/30"
                }`}
                style={{ backgroundColor: color }}
                onClick={() => {
                  setDraft(color);
                  setError(false);
                  commitColor(color);
                }}
              />
            );
          })}
          <button
            type="button"
            disabled={isUpdating}
            aria-label={t("settings.appearance.colors.resetColor")}
            className="w-8 h-8 rounded-md border-2 border-dashed border-mid-gray/40 text-xs text-mid-gray hover:border-logo-primary disabled:opacity-50"
            onClick={() => {
              setDraft("");
              setError(false);
              commitColor(null);
            }}
          >
            ×
          </button>
        </div>
        <div className="flex items-center gap-2">
          <input
            type="color"
            value={
              value && isValidHexColor(value)
                ? value
                : presets[0] ?? "#faa2ca"
            }
            disabled={isUpdating}
            className="w-10 h-10 rounded-md border border-mid-gray/30 bg-transparent cursor-pointer disabled:opacity-50"
            onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
              const next = event.target.value;
              setDraft(next);
              setError(false);
              commitColor(next);
            }}
          />
          <Input
            value={draft}
            disabled={isUpdating}
            placeholder={t("settings.appearance.colors.hexPlaceholder")}
            className={`flex-1 font-mono ${error ? "border-red-500" : ""}`}
            onChange={(event: React.ChangeEvent<HTMLInputElement>) =>
              setDraft(event.target.value)
            }
            onBlur={handleBlur}
            onKeyDown={(event: React.KeyboardEvent<HTMLInputElement>) => {
              if (event.key === "Enter") {
                handleBlur();
              }
            }}
          />
          <ResetButton
            onClick={() => {
              setDraft("");
              setError(false);
              commitColor(null);
            }}
            disabled={isUpdating || !value}
            ariaLabel={t("settings.appearance.colors.resetColor")}
          />
        </div>
      </div>
    </SettingContainer>
  );
};

export const AppearanceSettings: React.FC = () => {
  const { t } = useTranslation();
  const { getSetting, updateSetting, isUpdating, refreshSettings } =
    useSettings();
  const [isResetting, setIsResetting] = useState(false);
  const [shortcutIconPath, setShortcutIconPath] = useState<string | null>(null);
  const [isRegeneratingIcon, setIsRegeneratingIcon] = useState(false);
  const [isWindows, setIsWindows] = useState(false);

  useEffect(() => {
    setIsWindows(osType() === "windows");
  }, []);

  const fontScaleOptions: SelectOption[] = useMemo(
    () => [
      { value: "small", label: t("settings.appearance.fontScale.small") },
      { value: "medium", label: t("settings.appearance.fontScale.medium") },
      { value: "large", label: t("settings.appearance.fontScale.large") },
    ],
    [t],
  );

  const densityOptions: SelectOption[] = useMemo(
    () => [
      { value: "compact", label: t("settings.appearance.controlDensity.compact") },
      { value: "normal", label: t("settings.appearance.controlDensity.normal") },
      {
        value: "comfortable",
        label: t("settings.appearance.controlDensity.comfortable"),
      },
    ],
    [t],
  );

  const colorSchemeOptions: SelectOption[] = useMemo(
    () => [
      { value: "auto", label: t("settings.appearance.colorScheme.auto") },
      { value: "light", label: t("settings.appearance.colorScheme.light") },
      { value: "dark", label: t("settings.appearance.colorScheme.dark") },
    ],
    [t],
  );

  const handleResetAll = async () => {
    setIsResetting(true);
    try {
      await commands.resetAppearanceSettings();
      await refreshSettings();
    } finally {
      setIsResetting(false);
    }
  };

  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <SettingsGroup title={t("settings.appearance.title")}>
        <SettingContainer
          title={t("settings.appearance.colorScheme.title")}
          description={t("settings.appearance.colorScheme.description")}
          descriptionMode="tooltip"
          grouped
          layout="stacked"
        >
          <Select
            value={(getSetting("appearance_color_scheme") ?? "auto") as string}
            options={colorSchemeOptions}
            isClearable={false}
            disabled={isUpdating("appearance_color_scheme")}
            onChange={(value: string | null) =>
              updateSetting(
                "appearance_color_scheme",
                (value ?? "auto") as ColorScheme,
              )
            }
          />
        </SettingContainer>

        <div className="flex flex-col gap-2">
          <ColorPicker
            title={t("settings.appearance.colors.accent.title")}
            description={t("settings.appearance.colors.accent.description")}
            value={getSetting("appearance_accent_color")}
            presets={ACCENT_COLOR_PRESETS}
            isUpdating={isUpdating("appearance_accent_color")}
            onChange={(color) => updateSetting("appearance_accent_color", color)}
          />
          <p className="px-4 -mt-1 text-xs text-mid-gray leading-relaxed">
            {t("settings.appearance.colors.accent.exeIconHint")}
          </p>
          {isWindows && (
              <div className="px-4 flex flex-col gap-1">
                <Button
                  variant="secondary"
                  size="sm"
                  className="self-start"
                  disabled={isRegeneratingIcon}
                  onClick={async () => {
                    setIsRegeneratingIcon(true);
                    setShortcutIconPath(null);
                    try {
                      const result = await commands.regenerateBundleIcon();
                      if (result.status === "ok") {
                        setShortcutIconPath(result.data);
                      }
                    } finally {
                      setIsRegeneratingIcon(false);
                    }
                  }}
                >
                  {t("settings.appearance.colors.accent.regenerateShortcutIcon")}
                </Button>
                {shortcutIconPath && (
                  <p className="text-xs text-mid-gray break-all">
                    {t("settings.appearance.colors.accent.shortcutIconSaved", {
                      path: shortcutIconPath,
                    })}
                  </p>
                )}
              </div>
            )}
        </div>

        <ColorPicker
          title={t("settings.appearance.colors.background.title")}
          description={t("settings.appearance.colors.background.description")}
          value={getSetting("appearance_background_color")}
          presets={BACKGROUND_COLOR_PRESETS}
          isUpdating={isUpdating("appearance_background_color")}
          onChange={(color) =>
            updateSetting("appearance_background_color", color)
          }
        />

        <ColorPicker
          title={t("settings.appearance.colors.text.title")}
          description={t("settings.appearance.colors.text.description")}
          value={getSetting("appearance_text_color")}
          presets={TEXT_COLOR_PRESETS}
          isUpdating={isUpdating("appearance_text_color")}
          onChange={(color) => updateSetting("appearance_text_color", color)}
        />

        <SettingContainer
          title={t("settings.appearance.fontScale.title")}
          description={t("settings.appearance.fontScale.description")}
          descriptionMode="tooltip"
          grouped
          layout="stacked"
        >
          <Select
            value={(getSetting("appearance_font_scale") ?? "medium") as string}
            options={fontScaleOptions}
            isClearable={false}
            disabled={isUpdating("appearance_font_scale")}
            onChange={(value: string | null) =>
              updateSetting(
                "appearance_font_scale",
                (value ?? "medium") as FontSizeScale,
              )
            }
          />
        </SettingContainer>

        <SettingContainer
          title={t("settings.appearance.controlDensity.title")}
          description={t("settings.appearance.controlDensity.description")}
          descriptionMode="tooltip"
          grouped
          layout="stacked"
        >
          <Select
            value={
              (getSetting("appearance_control_density") ?? "normal") as string
            }
            options={densityOptions}
            isClearable={false}
            disabled={isUpdating("appearance_control_density")}
            onChange={(value: string | null) =>
              updateSetting(
                "appearance_control_density",
                (value ?? "normal") as ControlDensity,
              )
            }
          />
        </SettingContainer>

        <div className="px-4 py-3 flex justify-end border-t border-mid-gray/20">
          <Button
            variant="secondary"
            size="sm"
            disabled={isResetting}
            onClick={handleResetAll}
          >
            {t("settings.appearance.resetAll")}
          </Button>
        </div>
      </SettingsGroup>
    </div>
  );
};
