import { ChevronDown } from "lucide-react";
import { useSettingsStore } from "../../stores/settingsStore";

export function ModelSelector() {
  const { models, selectedModelId, setSelectedModel } = useSettingsStore();

  return (
    <div className="relative">
      <select
        value={selectedModelId}
        onChange={(e) => setSelectedModel(e.target.value)}
        className="appearance-none bg-[var(--bg-tertiary)] border border-[var(--border-color)] rounded-lg px-3 py-1.5 pr-8 text-sm text-[var(--text-primary)] cursor-pointer hover:bg-[var(--border-color)] transition-colors outline-none"
      >
        {models.length === 0 && (
          <option value={selectedModelId}>{selectedModelId}</option>
        )}
        {Object.entries(
          models.reduce<Record<string, typeof models>>((acc, model) => {
            (acc[model.provider] ??= []).push(model);
            return acc;
          }, {})
        ).map(([provider, providerModels]) => (
          <optgroup key={provider} label={provider.toUpperCase()}>
            {providerModels.map((model) => (
              <option key={model.id} value={model.id}>
                {model.name}
              </option>
            ))}
          </optgroup>
        ))}
      </select>
      <ChevronDown
        size={14}
        className="absolute right-2 top-1/2 -translate-y-1/2 text-[var(--text-secondary)] pointer-events-none"
      />
    </div>
  );
}
