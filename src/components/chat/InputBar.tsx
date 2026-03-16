import { useState, useRef, useEffect } from "react";
import { SendHorizontal, Square, Paperclip } from "lucide-react";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { FileAttachment } from "./FileAttachment";
import { useConversationStore } from "../../stores/conversationStore";
import { useSettingsStore } from "../../stores/settingsStore";
import type { AttachmentData } from "../../types/conversation";

export function InputBar() {
  const [input, setInput] = useState("");
  const [attachments, setAttachments] = useState<AttachmentData[]>([]);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const { sendMessage, isStreaming, activeConversationId } =
    useConversationStore();
  const { selectedModelId } = useSettingsStore();

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height =
        Math.min(textareaRef.current.scrollHeight, 200) + "px";
    }
  }, [input]);

  const handleSend = async () => {
    const trimmed = input.trim();
    if (!trimmed || isStreaming || !activeConversationId) return;
    setInput("");
    setAttachments([]);
    await sendMessage(trimmed, selectedModelId, attachments);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleAttach = async () => {
    try {
      const path = await open({
        multiple: false,
        filters: [
          {
            name: "Files",
            extensions: ["png", "jpg", "jpeg", "gif", "webp", "txt", "md", "pdf"],
          },
        ],
      });
      if (!path) return;

      const bytes = await readFile(path);
      const base64 = btoa(
        Array.from(bytes)
          .map((b) => String.fromCharCode(b))
          .join("")
      );
      const fileName = path.split(/[/\\]/).pop() || "file";
      const ext = fileName.split(".").pop()?.toLowerCase() || "";
      const mimeTypes: Record<string, string> = {
        png: "image/png",
        jpg: "image/jpeg",
        jpeg: "image/jpeg",
        gif: "image/gif",
        webp: "image/webp",
        txt: "text/plain",
        md: "text/markdown",
        pdf: "application/pdf",
      };

      setAttachments((prev) => [
        ...prev,
        {
          file_name: fileName,
          mime_type: mimeTypes[ext] || "application/octet-stream",
          data: base64,
        },
      ]);
    } catch (e) {
      console.error("Failed to attach file:", e);
    }
  };

  return (
    <div className="border-t border-[var(--border-color)] bg-[var(--bg-primary)] p-4">
      {attachments.length > 0 && (
        <div className="flex gap-2 mb-2 flex-wrap">
          {attachments.map((att, i) => (
            <FileAttachment
              key={i}
              attachment={att}
              onRemove={() =>
                setAttachments((prev) => prev.filter((_, j) => j !== i))
              }
            />
          ))}
        </div>
      )}
      <div className="flex items-end gap-2 bg-[var(--bg-secondary)] border border-[var(--border-color)] rounded-xl px-3 py-2">
        <button
          onClick={handleAttach}
          className="p-1.5 rounded-lg hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)] transition-colors flex-shrink-0 mb-0.5"
          title="Attach file"
        >
          <Paperclip size={18} />
        </button>
        <textarea
          ref={textareaRef}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Send a message..."
          rows={1}
          className="flex-1 bg-transparent resize-none outline-none text-sm text-[var(--text-primary)] placeholder-[var(--text-secondary)] max-h-[200px]"
          disabled={isStreaming}
        />
        <button
          onClick={isStreaming ? undefined : handleSend}
          disabled={!input.trim() && !isStreaming}
          className={`p-1.5 rounded-lg transition-colors flex-shrink-0 mb-0.5 ${
            isStreaming
              ? "text-[var(--danger)] hover:bg-red-500/10"
              : input.trim()
              ? "text-[var(--accent)] hover:bg-[var(--accent)]/10"
              : "text-[var(--text-secondary)] opacity-50"
          }`}
          title={isStreaming ? "Stop generating" : "Send message"}
        >
          {isStreaming ? <Square size={18} /> : <SendHorizontal size={18} />}
        </button>
      </div>
      <p className="text-[10px] text-[var(--text-secondary)] text-center mt-2">
        Press Enter to send, Shift+Enter for new line
      </p>
    </div>
  );
}
