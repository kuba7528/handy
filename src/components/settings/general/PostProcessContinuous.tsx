import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../../ui/ToggleSwitch";
import { useSettings } from "../../../hooks/useSettings";

interface PostProcessContinuousProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const PostProcessContinuous: React.FC<PostProcessContinuousProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const enabled = getSetting("post_process_continuous") ?? false;

    return (
      <ToggleSwitch
        checked={enabled}
        onChange={(value) => updateSetting("post_process_continuous", value)}
        isUpdating={isUpdating("post_process_continuous")}
        label={t("settings.general.postProcessContinuous.label")}
        description={t("settings.general.postProcessContinuous.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  },
);
