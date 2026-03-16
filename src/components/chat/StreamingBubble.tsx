import { Bot } from "lucide-react";
import { MarkdownRenderer } from "./MarkdownRenderer";

interface StreamingBubbleProps {
  content: string;
}

export function StreamingBubble({ content }: StreamingBubbleProps) {
  return (
    <div className="flex gap-3 px-4 py-4 bg-[var(--bg-secondary)]">
      <div className="flex-shrink-0 w-7 h-7 rounded-lg flex items-center justify-center bg-emerald-600 text-white">
        <Bot size={15} />
      </div>
      <div className="flex-1 min-w-0 overflow-hidden">
        <div className="flex items-center gap-2 mb-1">
          <span className="text-xs font-medium text-[var(--text-secondary)]">
            Assistant
          </span>
          <span className="inline-block w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
        </div>
        <div className="text-sm">
          {content ? (
            <MarkdownRenderer content={content} />
          ) : (
            <span className="text-[var(--text-secondary)]">Thinking...</span>
          )}
        </div>
      </div>
    </div>
  );
}
