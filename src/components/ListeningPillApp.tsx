import { useEffect } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import { commands } from "@/bindings";
import { useSettings } from "@/hooks/useSettings";
import { useAppearanceTheme } from "@/hooks/useAppearanceTheme";
import ListeningStatus from "./ListeningStatus";
import "../App.css";
import "./ListeningPillApp.css";

const PILL_POSITION_KEY = "listening-pill-position";

const ListeningPillApp: React.FC = () => {
  const { settings } = useSettings();
  useAppearanceTheme(settings);

  useEffect(() => {
    const pillWindow = getCurrentWebviewWindow();
    let unlistenMove: (() => void) | undefined;

    const restorePosition = async () => {
      try {
        const saved = localStorage.getItem(PILL_POSITION_KEY);
        if (!saved) return;
        const { x, y } = JSON.parse(saved) as { x: number; y: number };
        await pillWindow.setPosition(new PhysicalPosition(x, y));
      } catch (e) {
        console.warn("Failed to restore pill position:", e);
      }
    };

    const setupMoveListener = async () => {
      unlistenMove = await pillWindow.onMoved(({ payload }) => {
        localStorage.setItem(
          PILL_POSITION_KEY,
          JSON.stringify({ x: payload.x, y: payload.y }),
        );
      });
    };

    void restorePosition();
    void setupMoveListener();

    return () => {
      unlistenMove?.();
    };
  }, []);

  const handleExitCompactMode = async () => {
    try {
      await commands.exitListeningCompactMode();
    } catch (e) {
      console.warn("Failed to exit compact mode:", e);
    }
  };

  return (
    <div className="listening-pill-shell">
      <ListeningStatus variant="pill" onExitCompactMode={handleExitCompactMode} />
    </div>
  );
};

export default ListeningPillApp;
