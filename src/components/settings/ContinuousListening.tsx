import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { useSettings } from "../../hooks/useSettings";

interface ContinuousListeningProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const ContinuousListening: React.FC<ContinuousListeningProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const continuousListening = getSetting("continuous_listening") ?? true;

    return (
      <ToggleSwitch
        checked={continuousListening}
        onChange={(enabled) => updateSetting("continuous_listening", enabled)}
        isUpdating={isUpdating("continuous_listening")}
        label={t("settings.debug.continuousListening.label")}
        description={t("settings.debug.continuousListening.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  },
);
