import { ChatView } from "../chat/ChatView";
import { WelcomeScreen } from "../chat/WelcomeScreen";
import { SandboxPanel } from "../sandbox/SandboxPanel";
import { SplitPaneLayout } from "./SplitPaneLayout";
import { useConversationStore } from "../../stores/conversationStore";
import { useSandboxStore } from "../../stores/sandboxStore";

export function MainPanel() {
  const activeConversationId = useConversationStore(
    (s) => s.activeConversationId
  );
  const sandboxPanelOpen = useSandboxStore((s) => s.sandboxPanelOpen);

  return (
    <SplitPaneLayout
      left={activeConversationId ? <ChatView /> : <WelcomeScreen />}
      right={sandboxPanelOpen ? <SandboxPanel /> : null}
    />
  );
}
