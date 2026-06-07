import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type CSSProperties,
  type MouseEvent,
  type ReactNode,
} from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { Maximize2, Minus, SquareStack, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { commands } from "@/bindings";
import { useOsType } from "@/hooks/useOsType";
import { Tooltip } from "./ui/Tooltip";

const noDragStyle = { WebkitAppRegion: "no-drag" } as CSSProperties;
const dragStyle = { WebkitAppRegion: "drag" } as CSSProperties;

const startWindowDrag = (event: MouseEvent<HTMLElement>) => {
  if (event.button !== 0) return;
  void getCurrentWindow().startDragging();
};

const stopControlEvent = (event: MouseEvent<HTMLElement>) => {
  event.stopPropagation();
};

type WindowControl = "minimize" | "maximize" | "close";

interface ControlButtonProps {
  action: WindowControl;
  label: string;
  onClick: () => void;
  className?: string;
  icon: ReactNode;
}

function ControlButton({
  action,
  label,
  onClick,
  className = "",
  icon,
}: ControlButtonProps) {
  const buttonRef = useRef<HTMLButtonElement>(null);
  const [showTooltip, setShowTooltip] = useState(false);

  const hoverClass =
    action === "close"
      ? "hover:bg-red-600 hover:text-white"
      : "hover:bg-mid-gray/20";

  const handleClick = (event: MouseEvent<HTMLButtonElement>) => {
    stopControlEvent(event);
    onClick();
  };

  return (
    <>
      <button
        ref={buttonRef}
        type="button"
        data-tauri-drag-region="false"
        style={noDragStyle}
        aria-label={label}
        title={label}
        onMouseDown={stopControlEvent}
        onPointerDown={stopControlEvent}
        onClick={handleClick}
        onMouseEnter={() => setShowTooltip(true)}
        onMouseLeave={() => setShowTooltip(false)}
        onFocus={() => setShowTooltip(true)}
        onBlur={() => setShowTooltip(false)}
        className={`inline-flex h-8 w-11 items-center justify-center text-text/80 transition-colors ${hoverClass} ${className}`}
      >
        {icon}
      </button>
      {showTooltip && (
        <Tooltip targetRef={buttonRef} position="bottom">
          <span className="text-sm text-text">{label}</span>
        </Tooltip>
      )}
    </>
  );
}

function WindowControls() {
  const { t } = useTranslation();
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    const win = getCurrentWindow();
    let unlistenResize: (() => void) | undefined;

    const syncMaximized = async () => {
      try {
        setIsMaximized(await win.isMaximized());
      } catch {
        // ignore — window API may be unavailable during teardown
      }
    };

    void syncMaximized();
    void win.onResized(() => {
      void syncMaximized();
    }).then((unlisten) => {
      unlistenResize = unlisten;
    });

    return () => {
      unlistenResize?.();
    };
  }, []);

  const minimize = useCallback(async () => {
    const win = getCurrentWindow();
    try {
      await win.minimize();
      return;
    } catch (primaryError) {
      const result = await commands.minimizeMainWindowCommand();
      if (result.status === "error") {
        toast.error(t("window.minimizeFailed"), {
          description: String(result.error ?? primaryError),
        });
      }
    }
  }, [t]);

  const toggleMaximize = useCallback(async () => {
    const win = getCurrentWindow();
    try {
      await win.toggleMaximize();
      setIsMaximized(await win.isMaximized());
    } catch (error) {
      toast.error(t("window.maximizeFailed"), {
        description: String(error),
      });
    }
  }, [t]);

  const close = useCallback(async () => {
    try {
      const result = await commands.hideMainWindowToTrayCommand();
      if (result.status === "error") {
        throw new Error(String(result.error));
      }
      return;
    } catch (commandError) {
      try {
        await getCurrentWindow().hide();
      } catch (hideError) {
        toast.error(t("window.hideToTrayFailed"), {
          description: String(commandError ?? hideError),
        });
      }
    }
  }, [t]);

  const maximizeLabel = isMaximized
    ? t("window.restore")
    : t("window.maximize");

  return (
    <div
      className="relative z-20 flex shrink-0 items-stretch"
      data-tauri-drag-region="false"
      style={noDragStyle}
      onMouseDown={stopControlEvent}
      onPointerDown={stopControlEvent}
    >
      <ControlButton
        action="minimize"
        label={t("window.minimize")}
        onClick={() => {
          void minimize();
        }}
        icon={<Minus className="h-3.5 w-3.5" strokeWidth={2.25} />}
      />
      <ControlButton
        action="maximize"
        label={maximizeLabel}
        onClick={() => {
          void toggleMaximize();
        }}
        icon={
          isMaximized ? (
            <SquareStack className="h-3.5 w-3.5" strokeWidth={2.25} />
          ) : (
            <Maximize2 className="h-3.5 w-3.5" strokeWidth={2.25} />
          )
        }
      />
      <ControlButton
        action="close"
        label={t("window.closeTitle")}
        onClick={() => {
          void close();
        }}
        icon={<X className="h-3.5 w-3.5" strokeWidth={2.25} />}
      />
    </div>
  );
}

export default function TitleBar() {
  const { t } = useTranslation();
  const osType = useOsType();
  const controlsOnLeft = osType === "macos";

  const controls = <WindowControls />;
  const dragRegion = (
    <div
      data-tauri-drag-region="true"
      style={dragStyle}
      onMouseDown={startWindowDrag}
      className="flex h-full min-w-0 flex-1 items-center gap-2 self-stretch px-3"
    >
      <span className="pointer-events-none truncate text-xs font-medium tracking-wide text-text/70">
        {t("window.title")}
      </span>
    </div>
  );

  return (
    <header
      className="relative z-10 flex h-8 shrink-0 items-stretch border-b border-mid-gray/20 bg-background"
      data-tauri-drag-region="false"
      style={noDragStyle}
    >
      {controlsOnLeft ? (
        <>
          {controls}
          {dragRegion}
        </>
      ) : (
        <>
          {dragRegion}
          {controls}
        </>
      )}
    </header>
  );
}
