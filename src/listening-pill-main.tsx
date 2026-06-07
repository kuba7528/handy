import React from "react";
import ReactDOM from "react-dom/client";
import { platform } from "@tauri-apps/plugin-os";
import ListeningPillApp from "./components/ListeningPillApp";
import { useSettingsStore } from "./stores/settingsStore";

document.documentElement.dataset.platform = platform();

import "./i18n";

useSettingsStore.getState().initialize();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ListeningPillApp />
  </React.StrictMode>,
);
