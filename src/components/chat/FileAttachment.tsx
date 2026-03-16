import { Paperclip, X, Image, FileText } from "lucide-react";
import type { AttachmentData } from "../../types/conversation";

interface FileAttachmentProps {
  attachment: AttachmentData;
  onRemove: () => void;
}

export function FileAttachment({ attachment, onRemove }: FileAttachmentProps) {
  const isImage = attachment.mime_type.startsWith("image/");

  return (
    <div className="relative inline-flex items-center gap-2 bg-[var(--bg-tertiary)] rounded-lg px-3 py-2 text-sm">
      {isImage ? (
        <Image size={16} className="text-[var(--accent)]" />
      ) : (
        <FileText size={16} className="text-[var(--text-secondary)]" />
      )}
      <span className="max-w-[150px] truncate text-[var(--text-primary)]">
        {attachment.file_name}
      </span>
      <button
        onClick={onRemove}
        className="p-0.5 rounded hover:bg-[var(--border-color)] text-[var(--text-secondary)]"
      >
        <X size={14} />
      </button>
    </div>
  );
}

interface AttachButtonProps {
  onAttach: () => void;
}

export function AttachButton({ onAttach }: AttachButtonProps) {
  return (
    <button
      onClick={onAttach}
      className="p-2 rounded-lg hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)] transition-colors"
      title="Attach file"
    >
      <Paperclip size={18} />
    </button>
  );
}
