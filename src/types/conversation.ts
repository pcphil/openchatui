export interface Conversation {
  id: string;
  title: string;
  model_id: string;
  created_at: string;
  updated_at: string;
  archived: boolean;
}

export interface Message {
  id: string;
  conversation_id: string;
  role: "user" | "assistant" | "system";
  content: string;
  token_count: number | null;
  created_at: string;
  sort_order: number;
}

export interface Attachment {
  id: string;
  message_id: string;
  file_name: string;
  file_path: string;
  mime_type: string;
  file_size: number;
}

export interface AttachmentData {
  file_name: string;
  mime_type: string;
  data: string; // base64
}
