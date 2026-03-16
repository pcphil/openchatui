import { useEffect, useRef } from "react";
import { ArrowDown } from "lucide-react";
import { useState } from "react";
import { MessageBubble } from "./MessageBubble";
import { StreamingBubble } from "./StreamingBubble";
import { InputBar } from "./InputBar";
import { ModelSelector } from "../settings/ModelSelector";
import { useConversationStore } from "../../stores/conversationStore";

export function ChatView() {
  const { messages, streamingContent, isStreaming, error, clearError } =
    useConversationStore();
  const scrollRef = useRef<HTMLDivElement>(null);
  const bottomRef = useRef<HTMLDivElement>(null);
  const [showScrollButton, setShowScrollButton] = useState(false);

  // Auto-scroll to bottom on new messages
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, streamingContent]);

  // Show/hide scroll-to-bottom button
  const handleScroll = () => {
    if (!scrollRef.current) return;
    const { scrollTop, scrollHeight, clientHeight } = scrollRef.current;
    setShowScrollButton(scrollHeight - scrollTop - clientHeight > 100);
  };

  const scrollToBottom = () => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-[var(--border-color)]">
        <ModelSelector />
      </div>

      {/* Messages */}
      <div
        ref={scrollRef}
        onScroll={handleScroll}
        className="flex-1 overflow-y-auto"
      >
        {messages.length === 0 && !isStreaming && (
          <div className="flex items-center justify-center h-full text-[var(--text-secondary)] text-sm">
            Send a message to start the conversation
          </div>
        )}
        {messages.map((msg) => (
          <MessageBubble key={msg.id} message={msg} />
        ))}
        {isStreaming && <StreamingBubble content={streamingContent} />}
        {error && (
          <div className="mx-4 my-2 p-3 bg-red-500/10 border border-red-500/30 rounded-lg text-sm text-red-400">
            <p>{error}</p>
            <button
              onClick={clearError}
              className="text-xs underline mt-1 hover:text-red-300"
            >
              Dismiss
            </button>
          </div>
        )}
        <div ref={bottomRef} />
      </div>

      {/* Scroll to bottom */}
      {showScrollButton && (
        <div className="absolute bottom-32 left-1/2 -translate-x-1/2">
          <button
            onClick={scrollToBottom}
            className="p-2 rounded-full bg-[var(--bg-tertiary)] border border-[var(--border-color)] shadow-lg hover:bg-[var(--border-color)] transition-colors"
          >
            <ArrowDown size={16} />
          </button>
        </div>
      )}

      <InputBar />
    </div>
  );
}
