import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useTranslation } from "react-i18next";
import { toast, Toaster } from "sonner";
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
  const { t } = useTranslation();
  const { settings } = useSettings();
  useAppearanceTheme(settings);

  useEffect(() => {
    const unlistenCompactError = listen<string>(
      "listening-compact-mode-error",
      (event) => {
        toast.error(t("listeningStatus.compactMode.errorEnter"), {
          description: event.payload,
        });
      },
    );
    return () => {
      unlistenCompactError.then((fn) => fn());
    };
  }, [t]);

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
    const result = await commands.exitListeningCompactMode();
    if (result.status === "error") {
      toast.error(t("listeningStatus.compactMode.errorExit"), {
        description: String(result.error),
      });
    }
  };

  return (
    <>
      <Toaster
        theme="system"
        toastOptions={{
          unstyled: true,
          classNames: {
            toast:
              "bg-background border border-mid-gray/20 rounded-lg shadow-lg px-4 py-3 flex items-center gap-3 text-sm",
          },
        }}
      />
      <div className="listening-pill-shell">
        <ListeningStatus
          variant="pill"
          onExitCompactMode={handleExitCompactMode}
        />
      </div>
    </>
  );
};

export default ListeningPillApp;
