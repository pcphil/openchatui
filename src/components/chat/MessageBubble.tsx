import { User, Bot, Copy, Check } from "lucide-react";
import { useState } from "react";
import { MarkdownRenderer } from "./MarkdownRenderer";
import type { Message } from "../../types/conversation";

interface MessageBubbleProps {
  message: Message;
}

export function MessageBubble({ message }: MessageBubbleProps) {
  const [copied, setCopied] = useState(false);
  const isUser = message.role === "user";

  const handleCopy = () => {
    navigator.clipboard.writeText(message.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className={`group flex gap-3 px-4 py-4 ${isUser ? "" : "bg-[var(--bg-secondary)]"}`}>
      <div
        className={`flex-shrink-0 w-7 h-7 rounded-lg flex items-center justify-center ${
          isUser
            ? "bg-[var(--accent)] text-white"
            : "bg-emerald-600 text-white"
        }`}
      >
        {isUser ? <User size={15} /> : <Bot size={15} />}
      </div>
      <div className="flex-1 min-w-0 overflow-hidden">
        <div className="flex items-center gap-2 mb-1">
          <span className="text-xs font-medium text-[var(--text-secondary)]">
            {isUser ? "You" : "Assistant"}
          </span>
        </div>
        {isUser ? (
          <p className="whitespace-pre-wrap text-sm">{message.content}</p>
        ) : (
          <div className="text-sm">
            <MarkdownRenderer content={message.content} />
          </div>
        )}
      </div>
      <button
        onClick={handleCopy}
        className="flex-shrink-0 opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)] transition-opacity"
        title="Copy message"
      >
        {copied ? <Check size={14} /> : <Copy size={14} />}
      </button>
    </div>
  );
}
