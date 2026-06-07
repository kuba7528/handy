import { useCallback, useRef, useState, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { Minus, X } from "lucide-react";
import { commands } from "@/bindings";
import { useOsType } from "@/hooks/useOsType";
import { Tooltip } from "./ui/Tooltip";

type WindowControl = "minimize" | "close";

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

  return (
    <>
      <button
        ref={buttonRef}
        type="button"
        data-tauri-drag-region={false}
        aria-label={label}
        title={label}
        onClick={onClick}
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

  const minimize = useCallback(async () => {
    const result = await commands.minimizeMainWindowCommand();
    if (result.status === "error") {
      toast.error(t("window.minimizeFailed"), {
        description: String(result.error),
      });
    }
  }, [t]);

  const close = useCallback(async () => {
    const result = await commands.hideMainWindowToTrayCommand();
    if (result.status === "error") {
      toast.error(t("window.hideToTrayFailed"), {
        description: String(result.error),
      });
    }
  }, [t]);

  return (
    <div
      className="relative z-20 flex shrink-0 items-stretch"
      data-tauri-drag-region={false}
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
      data-tauri-drag-region
      className="flex min-w-0 flex-1 items-center gap-2 px-3"
    >
      <span className="truncate text-xs font-medium tracking-wide text-text/70">
        {t("window.title")}
      </span>
    </div>
  );

  return (
    <header className="relative z-10 flex h-8 shrink-0 items-stretch border-b border-mid-gray/20 bg-background">
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
