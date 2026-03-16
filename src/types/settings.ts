export interface Settings {
  openai_api_key?: string;
  anthropic_api_key?: string;
  google_api_key?: string;
  theme?: "dark" | "light";
  default_model?: string;
}
