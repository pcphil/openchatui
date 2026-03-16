import { useCallback, useEffect, useRef, useState, type ReactNode } from "react";

interface SplitPaneLayoutProps {
  left: ReactNode;
  right: ReactNode | null;
}

export function SplitPaneLayout({ left, right }: SplitPaneLayoutProps) {
  const [splitPercent, setSplitPercent] = useState(50);
  const isDragging = useRef(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    isDragging.current = true;
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
  }, []);

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!isDragging.current || !containerRef.current) return;
    const rect = containerRef.current.getBoundingClientRect();
    const percent = ((e.clientX - rect.left) / rect.width) * 100;
    setSplitPercent(Math.max(25, Math.min(75, percent)));
  }, []);

  const handleMouseUp = useCallback(() => {
    isDragging.current = false;
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }, []);

  useEffect(() => {
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
  }, [handleMouseMove, handleMouseUp]);

  if (!right) {
    return (
      <div className="flex-1 flex flex-col h-full overflow-hidden">{left}</div>
    );
  }

  return (
    <div ref={containerRef} className="flex-1 flex h-full overflow-hidden">
      <div
        className="flex flex-col overflow-hidden"
        style={{ width: `${splitPercent}%` }}
      >
        {left}
      </div>

      {/* Draggable divider */}
      <div
        onMouseDown={handleMouseDown}
        className="flex-shrink-0 flex items-center justify-center"
        style={{
          width: "6px",
          cursor: "col-resize",
          backgroundColor: "var(--border-color)",
          transition: isDragging.current ? "none" : "background-color 0.15s",
        }}
        onMouseEnter={(e) => {
          e.currentTarget.style.backgroundColor =
            "var(--accent, #3b82f6)";
        }}
        onMouseLeave={(e) => {
          if (!isDragging.current) {
            e.currentTarget.style.backgroundColor = "var(--border-color)";
          }
        }}
      >
        <div
          className="w-0.5 h-8 rounded-full"
          style={{ backgroundColor: "var(--text-secondary)", opacity: 0.5 }}
        />
      </div>

      <div
        className="flex flex-col overflow-hidden"
        style={{ width: `${100 - splitPercent}%` }}
      >
        {right}
      </div>
    </div>
  );
}
