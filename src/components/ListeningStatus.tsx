import { listen } from "@tauri-apps/api/event";
import React, { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { commands } from "@/bindings";
import { CancelIcon, MicrophoneIcon, TranscriptionIcon } from "./icons";

type Status = "idle" | "listening" | "recording" | "transcribing" | "processing";

const BAR_COUNT = 9;

const ListeningStatus: React.FC = () => {
  const { t } = useTranslation();
  const [status, setStatus] = useState<Status>("idle");
  const [levels, setLevels] = useState<number[]>(Array(BAR_COUNT).fill(0));
  const smoothedLevelsRef = useRef<number[]>(Array(BAR_COUNT).fill(0));

  useEffect(() => {
    let cancelled = false;

    const syncInitialStatus = async () => {
      try {
        const result = await commands.getListeningStatus();
        if (!cancelled && result.status === "ok") {
          setStatus(result.data as Status);
        }
      } catch (e) {
        console.warn("Failed to load listening status:", e);
      }
    };

    syncInitialStatus();

    const setupEventListeners = async () => {
      const unlistenStatus = await listen<string>("listening-status", (event) => {
        setStatus(event.payload as Status);
      });

      const unlistenLevel = await listen<number[]>("mic-level", (event) => {
        const newLevels = event.payload;
        const smoothed = smoothedLevelsRef.current.map((prev, i) => {
          const target = newLevels[i] || 0;
          return prev * 0.7 + target * 0.3;
        });
        smoothedLevelsRef.current = smoothed;
        setLevels(smoothed.slice(0, BAR_COUNT));
      });

      return () => {
        unlistenStatus();
        unlistenLevel();
      };
    };

    let cleanup: (() => void) | undefined;
    setupEventListeners().then((fn) => {
      cleanup = fn;
    });

    return () => {
      cancelled = true;
      cleanup?.();
    };
  }, []);

  if (status === "idle") {
    return null;
  }

  const showLevelBars = status === "listening" || status === "recording";

  const statusLabel =
    status === "listening"
      ? t("listeningStatus.listening")
      : status === "recording"
        ? t("listeningStatus.recording")
        : status === "transcribing"
          ? t("listeningStatus.transcribing")
          : t("listeningStatus.processing");

  const icon =
    status === "recording" ? (
      <MicrophoneIcon width={18} height={18} />
    ) : status === "transcribing" || status === "processing" ? (
      <TranscriptionIcon width={18} height={18} />
    ) : (
      <MicrophoneIcon width={18} height={18} />
    );

  return (
    <div
      className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-black/80 border border-mid-gray/30"
      role="status"
      aria-live="polite"
      aria-label={statusLabel}
    >
      <div className="flex items-center shrink-0">{icon}</div>

      <div className="flex items-center gap-2 min-w-0">
        {showLevelBars ? (
          <div className="flex items-end justify-center gap-0.5 h-5">
            {levels.map((v, i) => (
              <div
                key={i}
                className="w-1 rounded-sm bg-accent/90"
                style={{
                  height: `${Math.min(20, 4 + Math.pow(v, 0.7) * 16)}px`,
                  opacity: Math.max(0.25, v * 1.7),
                  transition: "height 60ms ease-out, opacity 120ms ease-out",
                }}
              />
            ))}
          </div>
        ) : (
          <span className="text-xs text-text/90 whitespace-nowrap animate-pulse">
            {statusLabel}
          </span>
        )}

        {showLevelBars && (
          <span className="text-xs text-text/70 whitespace-nowrap hidden sm:inline">
            {statusLabel}
          </span>
        )}
      </div>

      {status === "recording" && (
        <button
          type="button"
          className="flex items-center justify-center w-6 h-6 rounded-full shrink-0 hover:bg-accent/20 transition-colors"
          onClick={() => {
            commands.cancelOperation();
          }}
          aria-label={t("common.cancel")}
        >
          <CancelIcon />
        </button>
      )}
    </div>
  );
};

export default ListeningStatus;
