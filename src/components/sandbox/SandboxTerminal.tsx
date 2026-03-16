import { useEffect, useRef } from "react";

interface OutputLine {
  stream: "stdout" | "stderr";
  text: string;
  timestamp: number;
}

interface SandboxTerminalProps {
  lines: OutputLine[];
}

export function SandboxTerminal({ lines }: SandboxTerminalProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [lines]);

  return (
    <div
      ref={containerRef}
      className="flex-1 overflow-y-auto font-mono text-xs leading-5"
      style={{
        backgroundColor: "var(--bg-primary)",
        color: "var(--text-primary)",
        padding: "8px 12px",
      }}
    >
      {lines.length === 0 && (
        <div style={{ color: "var(--text-secondary)" }}>
          Waiting for output...
        </div>
      )}
      {lines.map((line, i) => (
        <div
          key={i}
          style={{
            color:
              line.stream === "stderr"
                ? "var(--danger, #ef4444)"
                : "var(--text-primary)",
            whiteSpace: "pre-wrap",
            wordBreak: "break-all",
          }}
        >
          {line.text}
        </div>
      ))}
    </div>
  );
}
