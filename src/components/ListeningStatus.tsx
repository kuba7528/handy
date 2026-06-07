import { listen } from "@tauri-apps/api/event";
import React, { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { commands } from "@/bindings";
import { CancelIcon, MicrophoneIcon, TranscriptionIcon } from "./icons";

type Status = "idle" | "listening" | "recording" | "transcribing" | "processing";

const LEVEL_BAR_COUNT = 16;
const WAVE_WIDTH = 72;
const WAVE_HEIGHT = 22;
const WAVE_POINTS = 48;

interface ListeningStatusProps {
  variant?: "footer" | "pill";
  onEnterCompactMode?: () => void;
  onExitCompactMode?: () => void;
}

function buildSineWavePath(
  phase: number,
  amplitude: number,
  width: number,
  height: number,
  points: number,
): string {
  const midY = height / 2;
  const maxAmp = (height / 2 - 2) * amplitude;
  const segments: string[] = [];

  for (let i = 0; i <= points; i++) {
    const x = (i / points) * width;
    const t = (i / points) * Math.PI * 4 + phase;
    const y = midY + Math.sin(t) * maxAmp;
    segments.push(`${i === 0 ? "M" : "L"} ${x.toFixed(2)} ${y.toFixed(2)}`);
  }

  return segments.join(" ");
}

interface SineWaveVisualizerProps {
  levels: number[];
  phase: number;
}

const SineWaveVisualizer: React.FC<SineWaveVisualizerProps> = ({
  levels,
  phase,
}) => {
  const avgLevel =
    levels.length > 0
      ? levels.reduce((sum, value) => sum + value, 0) / levels.length
      : 0;
  const peakLevel = levels.length > 0 ? Math.max(...levels) : 0;
  const amplitude = 0.22 + Math.pow(Math.max(avgLevel, peakLevel * 0.65), 0.65) * 0.78;
  const path = buildSineWavePath(
    phase,
    amplitude,
    WAVE_WIDTH,
    WAVE_HEIGHT,
    WAVE_POINTS,
  );

  return (
    <svg
      width={WAVE_WIDTH}
      height={WAVE_HEIGHT}
      viewBox={`0 0 ${WAVE_WIDTH} ${WAVE_HEIGHT}`}
      className="text-logo-primary shrink-0"
      aria-hidden="true"
    >
      <path
        d={path}
        fill="none"
        stroke="currentColor"
        strokeWidth={2}
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
};

const ListeningStatus: React.FC<ListeningStatusProps> = ({
  variant = "footer",
  onEnterCompactMode,
  onExitCompactMode,
}) => {
  const { t } = useTranslation();
  const [status, setStatus] = useState<Status>("idle");
  const [levels, setLevels] = useState<number[]>(Array(LEVEL_BAR_COUNT).fill(0));
  const smoothedLevelsRef = useRef<number[]>(Array(LEVEL_BAR_COUNT).fill(0));
  const statusRef = useRef<Status>("idle");
  const [wavePhase, setWavePhase] = useState(0);

  useEffect(() => {
    statusRef.current = status;
  }, [status]);

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
          const target = newLevels[i] ?? 0;
          return prev * 0.65 + target * 0.35;
        });
        smoothedLevelsRef.current = smoothed;
        setLevels(smoothed);
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

  useEffect(() => {
    if (status !== "listening" && status !== "recording") {
      return;
    }

    let frame = 0;
    let phase = 0;
    const animateWave = () => {
      const maxLevel = Math.max(...smoothedLevelsRef.current, 0);
      const speed = 0.06 + maxLevel * 0.14;
      phase += speed;
      setWavePhase(phase);

      if (statusRef.current === "listening" && maxLevel < 0.04) {
        const idleLevels = Array.from({ length: LEVEL_BAR_COUNT }, (_, i) => {
          const wave = Math.sin(phase + i * 0.4) * 0.5 + 0.5;
          return 0.06 + wave * 0.1;
        });
        smoothedLevelsRef.current = idleLevels;
        setLevels(idleLevels);
      }

      frame = window.requestAnimationFrame(animateWave);
    };

    frame = window.requestAnimationFrame(animateWave);
    return () => window.cancelAnimationFrame(frame);
  }, [status]);

  const handleEnterCompactMode = async () => {
    if (onEnterCompactMode) {
      onEnterCompactMode();
      return;
    }
    try {
      await commands.enterListeningCompactMode();
    } catch (e) {
      console.warn("Failed to enter compact mode:", e);
    }
  };

  const handleExitCompactMode = async () => {
    if (onExitCompactMode) {
      onExitCompactMode();
      return;
    }
    try {
      await commands.exitListeningCompactMode();
    } catch (e) {
      console.warn("Failed to exit compact mode:", e);
    }
  };

  const handleDoubleClick = (event: React.MouseEvent<HTMLDivElement>) => {
    if (event.button !== 0) return;
    if (variant === "pill") {
      void handleExitCompactMode();
    } else {
      void handleEnterCompactMode();
    }
  };

  if (status === "idle") {
    return null;
  }

  const showWave = status === "listening" || status === "recording";
  const isPill = variant === "pill";

  const statusLabel =
    status === "listening"
      ? t("listeningStatus.listening")
      : status === "recording"
        ? t("listeningStatus.recording")
        : status === "transcribing"
          ? t("listeningStatus.transcribing")
          : t("listeningStatus.processing");

  const compactHint =
    variant === "pill"
      ? t("listeningStatus.compactMode.exit")
      : t("listeningStatus.compactMode.enter");

  const iconClassName = "text-logo-primary";

  const icon =
    status === "recording" ? (
      <MicrophoneIcon width={18} height={18} className={iconClassName} />
    ) : status === "transcribing" || status === "processing" ? (
      <TranscriptionIcon width={18} height={18} className={iconClassName} />
    ) : (
      <MicrophoneIcon width={18} height={18} className={iconClassName} />
    );

  return (
    <div
      data-tauri-drag-region={isPill ? true : undefined}
      className={`flex items-center gap-2 px-3 py-1.5 rounded-full bg-black/80 border border-mid-gray/30 text-logo-primary ${
        isPill ? "cursor-grab active:cursor-grabbing select-none" : "cursor-default"
      }`}
      role="status"
      aria-live="polite"
      aria-label={statusLabel}
      title={compactHint}
      onDoubleClick={handleDoubleClick}
    >
      <div className="flex items-center shrink-0">{icon}</div>

      <div className="flex items-center gap-2 min-w-0">
        {showWave ? (
          <SineWaveVisualizer levels={levels} phase={wavePhase} />
        ) : (
          <span className="text-xs text-text/90 whitespace-nowrap animate-pulse">
            {statusLabel}
          </span>
        )}

        {showWave && (
          <span className="text-xs text-text/70 whitespace-nowrap hidden sm:inline">
            {statusLabel}
          </span>
        )}
      </div>

      {status === "recording" && (
        <button
          type="button"
          data-tauri-drag-region={false}
          className="flex items-center justify-center w-6 h-6 rounded-full shrink-0 hover:bg-logo-primary/20 transition-colors text-logo-primary"
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
