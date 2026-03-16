import { useEffect } from "react";
import { Sidebar } from "./components/layout/Sidebar";
import { MainPanel } from "./components/layout/MainPanel";
import { SettingsModal } from "./components/settings/SettingsModal";
import { useSettingsStore } from "./stores/settingsStore";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";

export default function App() {
  const { loadSettings, loadModels } = useSettingsStore();

  useEffect(() => {
    loadSettings();
    loadModels();
  }, [loadSettings, loadModels]);

  useKeyboardShortcuts();

  return (
    <div className="flex h-screen w-screen overflow-hidden">
      <Sidebar />
      <MainPanel />
      <SettingsModal />
    </div>
  );
}
