export interface Model {
  id: string;
  name: string;
  provider: string;
  supports_vision: boolean;
  supports_streaming: boolean;
}

export type StreamEvent =
  | { event: "Token"; data: string }
  | { event: "Done"; data: string }
  | { event: "Error"; data: string };
