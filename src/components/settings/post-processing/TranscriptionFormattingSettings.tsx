import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../../ui/ToggleSwitch";
import { useSettings } from "../../../hooks/useSettings";

export const TranscriptionFormattingSettings: React.FC = () => {
  const { t } = useTranslation();
  const { getSetting, updateSetting, isUpdating } = useSettings();

  const convertNumbers = getSetting("convert_spoken_numbers") ?? false;
  const convertSymbols = getSetting("convert_spoken_symbols") ?? false;

  return (
    <>
      <ToggleSwitch
        checked={convertNumbers}
        onChange={(enabled) => updateSetting("convert_spoken_numbers", enabled)}
        isUpdating={isUpdating("convert_spoken_numbers")}
        label={t("settings.advanced.formatting.numbers.label")}
        description={t("settings.advanced.formatting.numbers.description")}
        descriptionMode="tooltip"
        grouped={true}
      />
      <ToggleSwitch
        checked={convertSymbols}
        onChange={(enabled) => updateSetting("convert_spoken_symbols", enabled)}
        isUpdating={isUpdating("convert_spoken_symbols")}
        label={t("settings.advanced.formatting.symbols.label")}
        description={t("settings.advanced.formatting.symbols.description")}
        descriptionMode="tooltip"
        grouped={true}
      />
    </>
  );
};
