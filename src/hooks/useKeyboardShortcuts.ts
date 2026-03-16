import { useEffect } from "react";
import { useConversationStore } from "../stores/conversationStore";
import { useSettingsStore } from "../stores/settingsStore";
import { useUiStore } from "../stores/uiStore";

export function useKeyboardShortcuts() {
  const createConversation = useConversationStore((s) => s.createConversation);
  const selectedModelId = useSettingsStore((s) => s.selectedModelId);
  const toggleSidebar = useUiStore((s) => s.toggleSidebar);
  const toggleSettings = useSettingsStore((s) => s.toggleSettings);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+N: New chat
      if (e.ctrlKey && e.key === "n") {
        e.preventDefault();
        createConversation(undefined, selectedModelId);
      }
      // Ctrl+B: Toggle sidebar
      if (e.ctrlKey && e.key === "b") {
        e.preventDefault();
        toggleSidebar();
      }
      // Ctrl+,: Toggle settings
      if (e.ctrlKey && e.key === ",") {
        e.preventDefault();
        toggleSettings();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [createConversation, selectedModelId, toggleSidebar, toggleSettings]);
}
