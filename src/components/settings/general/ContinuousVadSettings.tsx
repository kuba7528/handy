import React from "react";
import { useTranslation } from "react-i18next";
import { Slider } from "../../ui/Slider";
import { useSettings } from "../../../hooks/useSettings";

interface ContinuousVadSettingsProps {
  descriptionMode?: "tooltip" | "inline";
  grouped?: boolean;
  disabled?: boolean;
}

export const ContinuousVadSettings: React.FC<ContinuousVadSettingsProps> = ({
  descriptionMode = "tooltip",
  grouped = false,
  disabled = false,
}) => {
  const { t } = useTranslation();
  const { settings, updateSetting } = useSettings();

  return (
    <>
      <Slider
        value={settings?.continuous_silence_ms ?? 720}
        onChange={(value) => updateSetting("continuous_silence_ms", Math.round(value))}
        min={200}
        max={3000}
        step={50}
        formatValue={(v) => `${Math.round(v)} ms`}
        label={t("settings.general.continuousVad.silenceMs.title")}
        description={t("settings.general.continuousVad.silenceMs.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        disabled={disabled}
      />
      <Slider
        value={settings?.continuous_min_segment_ms ?? 400}
        onChange={(value) =>
          updateSetting("continuous_min_segment_ms", Math.round(value))
        }
        min={100}
        max={3000}
        step={50}
        formatValue={(v) => `${Math.round(v)} ms`}
        label={t("settings.general.continuousVad.minSegmentMs.title")}
        description={t("settings.general.continuousVad.minSegmentMs.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        disabled={disabled}
      />
      <Slider
        value={settings?.vad_sensitivity ?? 0.3}
        onChange={(value) => updateSetting("vad_sensitivity", value)}
        min={0.05}
        max={0.95}
        step={0.05}
        label={t("settings.general.continuousVad.sensitivity.title")}
        description={t("settings.general.continuousVad.sensitivity.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
        disabled={disabled}
      />
    </>
  );
};
