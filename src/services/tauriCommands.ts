import { invoke, Channel } from "@tauri-apps/api/core";
import type { Conversation, Message, AttachmentData } from "../types/conversation";
import type { Model, StreamEvent } from "../types/provider";

// Conversations
export async function createConversation(
  title?: string,
  modelId?: string
): Promise<Conversation> {
  return invoke("create_conversation", { title, modelId });
}

export async function listConversations(): Promise<Conversation[]> {
  return invoke("list_conversations");
}

export async function getConversation(id: string): Promise<Conversation> {
  return invoke("get_conversation", { id });
}

export async function updateConversation(
  id: string,
  title?: string,
  modelId?: string,
  archived?: boolean
): Promise<Conversation> {
  return invoke("update_conversation", { id, title, modelId, archived });
}

export async function deleteConversation(id: string): Promise<void> {
  return invoke("delete_conversation", { id });
}

// Messages
export async function getMessages(conversationId: string): Promise<Message[]> {
  return invoke("get_messages", { conversationId });
}

export async function addMessage(
  conversationId: string,
  role: string,
  content: string
): Promise<Message> {
  return invoke("add_message", { conversationId, role, content });
}

// Chat
export async function sendMessage(
  conversationId: string,
  content: string,
  modelId: string,
  attachments: AttachmentData[],
  onEvent: (event: StreamEvent) => void
): Promise<string> {
  const channel = new Channel<StreamEvent>();
  channel.onmessage = onEvent;
  return invoke("send_message", {
    conversationId,
    content,
    modelId,
    attachments,
    onEvent: channel,
  });
}

// Models
export async function listModels(): Promise<Model[]> {
  return invoke("list_models");
}

export async function testProviderConnection(
  provider: string
): Promise<boolean> {
  return invoke("test_provider_connection", { provider });
}

// Settings
export async function getSetting(key: string): Promise<string | null> {
  return invoke("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invoke("set_setting", { key, value });
}

export async function getAllSettings(): Promise<Record<string, string>> {
  return invoke("get_all_settings");
}
