import { create } from "zustand";
import type { Conversation, Message, AttachmentData } from "../types/conversation";
import * as api from "../services/tauriCommands";
import type { StreamEvent } from "../types/provider";

interface ConversationState {
  conversations: Conversation[];
  activeConversationId: string | null;
  messages: Message[];
  streamingContent: string;
  isStreaming: boolean;
  error: string | null;

  loadConversations: () => Promise<void>;
  createConversation: (title?: string, modelId?: string) => Promise<Conversation>;
  selectConversation: (id: string) => Promise<void>;
  updateConversation: (id: string, title?: string, modelId?: string) => Promise<void>;
  deleteConversation: (id: string) => Promise<void>;
  sendMessage: (content: string, modelId: string, attachments?: AttachmentData[]) => Promise<void>;
  clearError: () => void;
}

export const useConversationStore = create<ConversationState>((set, get) => ({
  conversations: [],
  activeConversationId: null,
  messages: [],
  streamingContent: "",
  isStreaming: false,
  error: null,

  loadConversations: async () => {
    try {
      const conversations = await api.listConversations();
      set({ conversations });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  createConversation: async (title?: string, modelId?: string) => {
    try {
      const conversation = await api.createConversation(title, modelId);
      set((state) => ({
        conversations: [conversation, ...state.conversations],
        activeConversationId: conversation.id,
        messages: [],
      }));
      return conversation;
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  selectConversation: async (id: string) => {
    try {
      const messages = await api.getMessages(id);
      set({ activeConversationId: id, messages, streamingContent: "", error: null });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  updateConversation: async (id: string, title?: string, modelId?: string) => {
    try {
      const updated = await api.updateConversation(id, title, modelId);
      set((state) => ({
        conversations: state.conversations.map((c) =>
          c.id === id ? updated : c
        ),
      }));
    } catch (e) {
      set({ error: String(e) });
    }
  },

  deleteConversation: async (id: string) => {
    try {
      await api.deleteConversation(id);
      set((state) => {
        const conversations = state.conversations.filter((c) => c.id !== id);
        const newActiveId =
          state.activeConversationId === id
            ? conversations[0]?.id ?? null
            : state.activeConversationId;
        return {
          conversations,
          activeConversationId: newActiveId,
          messages: state.activeConversationId === id ? [] : state.messages,
        };
      });
      // Load messages for new active conversation
      const { activeConversationId } = get();
      if (activeConversationId) {
        const messages = await api.getMessages(activeConversationId);
        set({ messages });
      }
    } catch (e) {
      set({ error: String(e) });
    }
  },

  sendMessage: async (
    content: string,
    modelId: string,
    attachments: AttachmentData[] = []
  ) => {
    const { activeConversationId } = get();
    if (!activeConversationId) return;

    // Optimistically add user message
    const userMessage: Message = {
      id: crypto.randomUUID(),
      conversation_id: activeConversationId,
      role: "user",
      content,
      token_count: null,
      created_at: new Date().toISOString(),
      sort_order: get().messages.length,
    };

    set((state) => ({
      messages: [...state.messages, userMessage],
      streamingContent: "",
      isStreaming: true,
      error: null,
    }));

    try {
      await api.sendMessage(
        activeConversationId,
        content,
        modelId,
        attachments,
        (event: StreamEvent) => {
          switch (event.event) {
            case "Token":
              set((state) => ({
                streamingContent: state.streamingContent + event.data,
              }));
              break;
            case "Done": {
              const assistantMessage: Message = {
                id: crypto.randomUUID(),
                conversation_id: activeConversationId,
                role: "assistant",
                content: event.data,
                token_count: null,
                created_at: new Date().toISOString(),
                sort_order: get().messages.length,
              };
              set((state) => ({
                messages: [...state.messages, assistantMessage],
                streamingContent: "",
                isStreaming: false,
              }));
              break;
            }
            case "Error":
              set({ error: event.data, isStreaming: false, streamingContent: "" });
              break;
          }
        }
      );
    } catch (e) {
      set({ error: String(e), isStreaming: false, streamingContent: "" });
    }
  },

  clearError: () => set({ error: null }),
}));
