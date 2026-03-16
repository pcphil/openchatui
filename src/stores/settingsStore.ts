import { create } from "zustand";
import * as api from "../services/tauriCommands";
import type { Model } from "../types/provider";

interface SettingsState {
  models: Model[];
  selectedModelId: string;
  theme: "dark" | "light";
  settingsOpen: boolean;
  apiKeys: Record<string, string>;

  loadModels: () => Promise<void>;
  setSelectedModel: (id: string) => void;
  setTheme: (theme: "dark" | "light") => void;
  toggleSettings: () => void;
  setApiKey: (provider: string, key: string) => Promise<void>;
  loadSettings: () => Promise<void>;
  testConnection: (provider: string) => Promise<boolean>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  models: [],
  selectedModelId: "ollama:llama3",
  theme: "dark",
  settingsOpen: false,
  apiKeys: {},

  loadModels: async () => {
    try {
      const models = await api.listModels();
      set({ models });
      // If no model selected and models available, pick first
      if (models.length > 0 && !models.find((m) => m.id === get().selectedModelId)) {
        set({ selectedModelId: models[0].id });
      }
    } catch (e) {
      console.error("Failed to load models:", e);
    }
  },

  setSelectedModel: (id: string) => set({ selectedModelId: id }),

  setTheme: (theme: "dark" | "light") => {
    set({ theme });
    document.documentElement.classList.toggle("dark", theme === "dark");
    api.setSetting("theme", theme).catch(console.error);
  },

  toggleSettings: () => set((state) => ({ settingsOpen: !state.settingsOpen })),

  setApiKey: async (provider: string, key: string) => {
    await api.setSetting(`${provider}_api_key`, key);
    set((state) => ({
      apiKeys: { ...state.apiKeys, [provider]: key ? "configured" : "" },
    }));
    // Reload models after configuring a provider
    get().loadModels();
  },

  loadSettings: async () => {
    try {
      const settings = await api.getAllSettings();
      const theme = (settings.theme as "dark" | "light") || "dark";
      document.documentElement.classList.toggle("dark", theme === "dark");
      set({
        theme,
        selectedModelId: settings.default_model || "ollama:llama3",
        apiKeys: {
          openai: settings.openai_api_key || "",
          anthropic: settings.anthropic_api_key || "",
          google: settings.google_api_key || "",
        },
      });
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  },

  testConnection: async (provider: string) => {
    return api.testProviderConnection(provider);
  },
}));
