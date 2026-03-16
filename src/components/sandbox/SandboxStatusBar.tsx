import type { SandboxStatus } from "../../types/sandbox";

interface SandboxStatusBarProps {
  status: SandboxStatus;
  onStop: () => void;
  onDestroy: () => void;
  onClose: () => void;
}

const STATUS_COLORS: Record<SandboxStatus, string> = {
  creating: "#f59e0b",
  running: "#22c55e",
  stopped: "#6b7280",
  failed: "#ef4444",
  destroyed: "#6b7280",
};

const STATUS_LABELS: Record<SandboxStatus, string> = {
  creating: "Creating",
  running: "Running",
  stopped: "Stopped",
  failed: "Failed",
  destroyed: "Destroyed",
};

export function SandboxStatusBar({
  status,
  onStop,
  onDestroy,
  onClose,
}: SandboxStatusBarProps) {
  const color = STATUS_COLORS[status];

  return (
    <div
      className="flex items-center justify-between px-3 py-2 text-xs"
      style={{
        backgroundColor: "var(--bg-tertiary)",
        borderBottom: "1px solid var(--border-color)",
      }}
    >
      <div className="flex items-center gap-2">
        <span className="font-semibold" style={{ color: "var(--text-primary)" }}>
          Sandbox
        </span>
        <span className="flex items-center gap-1">
          <span
            className="inline-block w-2 h-2 rounded-full"
            style={{
              backgroundColor: color,
              boxShadow: status === "running" ? `0 0 6px ${color}` : "none",
            }}
          />
          <span style={{ color }}>{STATUS_LABELS[status]}</span>
        </span>
      </div>

      <div className="flex items-center gap-1">
        {status === "running" && (
          <button
            onClick={onStop}
            className="px-2 py-0.5 rounded text-xs transition-colors"
            style={{
              color: "var(--text-secondary)",
              backgroundColor: "transparent",
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = "var(--bg-secondary)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = "transparent";
            }}
          >
            Stop
          </button>
        )}
        {(status === "stopped" || status === "failed") && (
          <button
            onClick={onDestroy}
            className="px-2 py-0.5 rounded text-xs transition-colors"
            style={{
              color: "var(--danger, #ef4444)",
              backgroundColor: "transparent",
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = "var(--bg-secondary)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = "transparent";
            }}
          >
            Destroy
          </button>
        )}
        <button
          onClick={onClose}
          className="px-2 py-0.5 rounded text-xs transition-colors"
          style={{
            color: "var(--text-secondary)",
            backgroundColor: "transparent",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = "var(--bg-secondary)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          Close
        </button>
      </div>
    </div>
  );
}
