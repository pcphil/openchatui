import { useState } from "react";
import { Modal } from "../common/Modal";
import { Button } from "../common/Button";
import { useSettingsStore } from "../../stores/settingsStore";
import { Sun, Moon, CheckCircle, XCircle, Loader2 } from "lucide-react";

export function SettingsModal() {
  const {
    settingsOpen,
    toggleSettings,
    theme,
    setTheme,
    setApiKey,
    apiKeys,
    testConnection,
  } = useSettingsStore();

  return (
    <Modal open={settingsOpen} onClose={toggleSettings} title="Settings">
      <div className="space-y-6">
        {/* Theme */}
        <div>
          <label className="text-sm font-medium mb-2 block">Theme</label>
          <div className="flex gap-2">
            <Button
              variant={theme === "dark" ? "primary" : "secondary"}
              size="sm"
              onClick={() => setTheme("dark")}
            >
              <Moon size={14} /> Dark
            </Button>
            <Button
              variant={theme === "light" ? "primary" : "secondary"}
              size="sm"
              onClick={() => setTheme("light")}
            >
              <Sun size={14} /> Light
            </Button>
          </div>
        </div>

        {/* API Keys */}
        <div>
          <label className="text-sm font-medium mb-3 block">API Keys</label>
          <div className="space-y-3">
            <ApiKeyInput
              label="OpenAI"
              placeholder="sk-..."
              configured={!!apiKeys.openai}
              onSave={(key) => setApiKey("openai", key)}
              onTest={() => testConnection("openai")}
            />
            <ApiKeyInput
              label="Anthropic"
              placeholder="sk-ant-..."
              configured={!!apiKeys.anthropic}
              onSave={(key) => setApiKey("anthropic", key)}
              onTest={() => testConnection("anthropic")}
            />
            <ApiKeyInput
              label="Google AI"
              placeholder="AI..."
              configured={!!apiKeys.google}
              onSave={(key) => setApiKey("google", key)}
              onTest={() => testConnection("google")}
            />
          </div>
        </div>

        {/* Info */}
        <p className="text-xs text-[var(--text-secondary)]">
          API keys are stored locally and never leave your device. They are sent
          directly to the provider APIs from your machine.
        </p>
      </div>
    </Modal>
  );
}

interface ApiKeyInputProps {
  label: string;
  placeholder: string;
  configured: boolean;
  onSave: (key: string) => Promise<void>;
  onTest: () => Promise<boolean>;
}

function ApiKeyInput({
  label,
  placeholder,
  configured,
  onSave,
  onTest,
}: ApiKeyInputProps) {
  const [value, setValue] = useState("");
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<boolean | null>(null);

  const handleSave = async () => {
    if (value.trim()) {
      await onSave(value.trim());
      setValue("");
    }
  };

  const handleTest = async () => {
    setTesting(true);
    setTestResult(null);
    try {
      const result = await onTest();
      setTestResult(result);
    } catch {
      setTestResult(false);
    }
    setTesting(false);
    setTimeout(() => setTestResult(null), 3000);
  };

  return (
    <div className="space-y-1">
      <div className="flex items-center justify-between">
        <label className="text-xs text-[var(--text-secondary)]">{label}</label>
        {configured && (
          <div className="flex items-center gap-1">
            <span className="text-xs text-emerald-500">Configured</span>
            <button
              onClick={handleTest}
              className="text-xs text-[var(--accent)] hover:underline"
              disabled={testing}
            >
              {testing ? (
                <Loader2 size={12} className="animate-spin" />
              ) : testResult === true ? (
                <CheckCircle size={12} className="text-emerald-500" />
              ) : testResult === false ? (
                <XCircle size={12} className="text-red-500" />
              ) : (
                "Test"
              )}
            </button>
          </div>
        )}
      </div>
      <div className="flex gap-2">
        <input
          type="password"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          placeholder={configured ? "••••••••" : placeholder}
          className="flex-1 bg-[var(--bg-tertiary)] border border-[var(--border-color)] rounded-lg px-3 py-1.5 text-sm outline-none focus:border-[var(--accent)]"
          onKeyDown={(e) => e.key === "Enter" && handleSave()}
        />
        <Button size="sm" variant="secondary" onClick={handleSave} disabled={!value.trim()}>
          Save
        </Button>
      </div>
    </div>
  );
}
