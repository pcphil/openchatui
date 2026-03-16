interface DiffViewerProps {
  diff: string;
}

export function DiffViewer({ diff }: DiffViewerProps) {
  const lines = diff.split("\n");

  return (
    <div
      className="font-mono text-xs overflow-x-auto rounded"
      style={{
        backgroundColor: "var(--bg-primary)",
        border: "1px solid var(--border-color)",
      }}
    >
      {lines.map((line, i) => {
        let bgColor = "transparent";
        let textColor = "var(--text-primary)";

        if (line.startsWith("+") && !line.startsWith("+++")) {
          bgColor = "rgba(34, 197, 94, 0.15)";
          textColor = "#22c55e";
        } else if (line.startsWith("-") && !line.startsWith("---")) {
          bgColor = "rgba(239, 68, 68, 0.15)";
          textColor = "#ef4444";
        } else if (line.startsWith("@@")) {
          textColor = "var(--accent, #3b82f6)";
        }

        return (
          <div
            key={i}
            style={{
              backgroundColor: bgColor,
              color: textColor,
              padding: "0 12px",
              lineHeight: "20px",
              whiteSpace: "pre",
            }}
          >
            <span
              style={{
                display: "inline-block",
                width: "32px",
                textAlign: "right",
                marginRight: "12px",
                color: "var(--text-secondary)",
                userSelect: "none",
              }}
            >
              {i + 1}
            </span>
            {line}
          </div>
        );
      })}
    </div>
  );
}
