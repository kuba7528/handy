import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { useSettings } from "../../hooks/useSettings";

interface AppendTrailingPeriodProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const AppendTrailingPeriod: React.FC<AppendTrailingPeriodProps> =
  React.memo(({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const enabled = getSetting("append_trailing_period") ?? false;

    return (
      <ToggleSwitch
        checked={enabled}
        onChange={(enabled) => updateSetting("append_trailing_period", enabled)}
        isUpdating={isUpdating("append_trailing_period")}
        label={t("settings.advanced.formatting.trailingPeriod.label")}
        description={t("settings.advanced.formatting.trailingPeriod.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  });
