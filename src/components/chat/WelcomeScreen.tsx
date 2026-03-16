import { MessageSquare, Zap, Shield, Globe } from "lucide-react";
import { useConversationStore } from "../../stores/conversationStore";
import { useSettingsStore } from "../../stores/settingsStore";

const suggestions = [
  "Explain quantum computing in simple terms",
  "Write a Python function to find prime numbers",
  "What are the pros and cons of TypeScript?",
  "Help me debug a React useEffect issue",
];

export function WelcomeScreen() {
  const { createConversation, sendMessage } = useConversationStore();
  const { selectedModelId } = useSettingsStore();

  const handleSuggestion = async (text: string) => {
    const convo = await createConversation(undefined, selectedModelId);
    if (convo) {
      await sendMessage(text, selectedModelId);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center h-full px-4">
      <div className="max-w-2xl w-full">
        <h1 className="text-3xl font-bold text-center mb-2">OpenChatUI</h1>
        <p className="text-[var(--text-secondary)] text-center mb-10">
          Your local AI chat interface. Connect to Ollama, OpenAI, Anthropic, or
          Google AI.
        </p>

        {/* Features */}
        <div className="grid grid-cols-2 gap-3 mb-10">
          {[
            { icon: MessageSquare, label: "Multi-model chat", desc: "Switch between providers" },
            { icon: Zap, label: "Streaming responses", desc: "Real-time token rendering" },
            { icon: Shield, label: "Local first", desc: "API keys stay on device" },
            { icon: Globe, label: "4 providers", desc: "Ollama, OpenAI, Anthropic, Google" },
          ].map(({ icon: Icon, label, desc }) => (
            <div
              key={label}
              className="flex items-start gap-3 p-3 rounded-xl bg-[var(--bg-secondary)] border border-[var(--border-color)]"
            >
              <Icon size={20} className="text-[var(--accent)] mt-0.5 flex-shrink-0" />
              <div>
                <p className="text-sm font-medium">{label}</p>
                <p className="text-xs text-[var(--text-secondary)]">{desc}</p>
              </div>
            </div>
          ))}
        </div>

        {/* Suggestions */}
        <p className="text-sm text-[var(--text-secondary)] mb-3 text-center">
          Try asking...
        </p>
        <div className="grid grid-cols-2 gap-2">
          {suggestions.map((text) => (
            <button
              key={text}
              onClick={() => handleSuggestion(text)}
              className="text-left p-3 rounded-xl border border-[var(--border-color)] hover:bg-[var(--bg-secondary)] transition-colors text-sm text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
            >
              {text}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
