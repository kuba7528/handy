import React from "react";
import ReactDOM from "react-dom/client";
import { platform } from "@tauri-apps/plugin-os";
import ListeningPillApp from "./components/ListeningPillApp";

document.documentElement.dataset.platform = platform();

import "./i18n";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ListeningPillApp />
  </React.StrictMode>,
);
