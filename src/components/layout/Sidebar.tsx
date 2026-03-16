import { useEffect, useState } from "react";
import {
  MessageSquarePlus,
  Trash2,
  Settings,
  PanelLeftClose,
  PanelLeft,
  Pencil,
  Check,
  X,
} from "lucide-react";
import { useConversationStore } from "../../stores/conversationStore";
import { useSettingsStore } from "../../stores/settingsStore";
import { useUiStore } from "../../stores/uiStore";

export function Sidebar() {
  const {
    conversations,
    activeConversationId,
    loadConversations,
    createConversation,
    selectConversation,
    updateConversation,
    deleteConversation,
  } = useConversationStore();
  const { toggleSettings, selectedModelId } = useSettingsStore();
  const { sidebarOpen, toggleSidebar } = useUiStore();
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editTitle, setEditTitle] = useState("");

  useEffect(() => {
    loadConversations();
  }, [loadConversations]);

  const handleNewChat = async () => {
    await createConversation(undefined, selectedModelId);
  };

  const handleRename = (id: string, currentTitle: string) => {
    setEditingId(id);
    setEditTitle(currentTitle);
  };

  const handleSaveRename = async () => {
    if (editingId && editTitle.trim()) {
      await updateConversation(editingId, editTitle.trim());
    }
    setEditingId(null);
  };

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    await deleteConversation(id);
  };

  if (!sidebarOpen) {
    return (
      <div className="flex flex-col items-center py-3 px-1 border-r border-[var(--border-color)] bg-[var(--bg-secondary)]">
        <button
          onClick={toggleSidebar}
          className="p-2 rounded-lg hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)]"
          title="Open sidebar"
        >
          <PanelLeft size={20} />
        </button>
        <button
          onClick={handleNewChat}
          className="p-2 mt-2 rounded-lg hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)]"
          title="New chat"
        >
          <MessageSquarePlus size={20} />
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col w-64 border-r border-[var(--border-color)] bg-[var(--bg-secondary)] h-full">
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b border-[var(--border-color)]">
        <h1 className="text-sm font-semibold">OpenChatUI</h1>
        <div className="flex gap-1">
          <button
            onClick={handleNewChat}
            className="p-1.5 rounded-lg hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)]"
            title="New chat (Ctrl+N)"
          >
            <MessageSquarePlus size={18} />
          </button>
          <button
            onClick={toggleSidebar}
            className="p-1.5 rounded-lg hover:bg-[var(--bg-tertiary)] text-[var(--text-secondary)]"
            title="Close sidebar"
          >
            <PanelLeftClose size={18} />
          </button>
        </div>
      </div>

      {/* Conversation List */}
      <div className="flex-1 overflow-y-auto py-2">
        {conversations.length === 0 ? (
          <p className="text-center text-[var(--text-secondary)] text-sm mt-8">
            No conversations yet
          </p>
        ) : (
          conversations.map((convo) => (
            <div
              key={convo.id}
              onClick={() => selectConversation(convo.id)}
              className={`group flex items-center gap-2 mx-2 px-3 py-2 rounded-lg cursor-pointer text-sm transition-colors ${
                activeConversationId === convo.id
                  ? "bg-[var(--bg-tertiary)] text-[var(--text-primary)]"
                  : "text-[var(--text-secondary)] hover:bg-[var(--bg-tertiary)]"
              }`}
            >
              {editingId === convo.id ? (
                <div className="flex items-center gap-1 flex-1 min-w-0">
                  <input
                    value={editTitle}
                    onChange={(e) => setEditTitle(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") handleSaveRename();
                      if (e.key === "Escape") setEditingId(null);
                    }}
                    className="flex-1 min-w-0 bg-[var(--bg-primary)] border border-[var(--border-color)] rounded px-1 py-0.5 text-sm"
                    autoFocus
                    onClick={(e) => e.stopPropagation()}
                  />
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleSaveRename();
                    }}
                    className="p-0.5 hover:text-green-500"
                  >
                    <Check size={14} />
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      setEditingId(null);
                    }}
                    className="p-0.5 hover:text-red-500"
                  >
                    <X size={14} />
                  </button>
                </div>
              ) : (
                <>
                  <span className="flex-1 truncate">{convo.title}</span>
                  <div className="hidden group-hover:flex items-center gap-0.5">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleRename(convo.id, convo.title);
                      }}
                      className="p-0.5 hover:text-[var(--accent)]"
                    >
                      <Pencil size={13} />
                    </button>
                    <button
                      onClick={(e) => handleDelete(e, convo.id)}
                      className="p-0.5 hover:text-[var(--danger)]"
                    >
                      <Trash2 size={13} />
                    </button>
                  </div>
                </>
              )}
            </div>
          ))
        )}
      </div>

      {/* Footer */}
      <div className="p-3 border-t border-[var(--border-color)]">
        <button
          onClick={toggleSettings}
          className="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-sm text-[var(--text-secondary)] hover:bg-[var(--bg-tertiary)] transition-colors"
        >
          <Settings size={16} />
          Settings
        </button>
      </div>
    </div>
  );
}
