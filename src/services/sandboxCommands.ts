import { invoke, Channel } from "@tauri-apps/api/core";
import type {
  SandboxConfig,
  SandboxEvent,
  SandboxInfo,
} from "../types/sandbox";

export async function createSandbox(
  conversationId: string,
  config: SandboxConfig | null,
  onEvent: (event: SandboxEvent) => void
): Promise<SandboxInfo> {
  const channel = new Channel<SandboxEvent>();
  channel.onmessage = onEvent;
  return invoke("create_sandbox", {
    conversationId,
    config,
    onEvent: channel,
  });
}

export async function execInSandbox(
  sandboxId: string,
  command: string[],
  onEvent: (event: SandboxEvent) => void
): Promise<number> {
  const channel = new Channel<SandboxEvent>();
  channel.onmessage = onEvent;
  return invoke("exec_in_sandbox", {
    sandboxId,
    command,
    onEvent: channel,
  });
}

export async function approveProposal(
  proposalId: string,
  onEvent: (event: SandboxEvent) => void
): Promise<void> {
  const channel = new Channel<SandboxEvent>();
  channel.onmessage = onEvent;
  return invoke("approve_proposal", {
    proposalId,
    onEvent: channel,
  });
}

export async function rejectProposal(
  proposalId: string,
  onEvent: (event: SandboxEvent) => void
): Promise<void> {
  const channel = new Channel<SandboxEvent>();
  channel.onmessage = onEvent;
  return invoke("reject_proposal", {
    proposalId,
    onEvent: channel,
  });
}

export async function stopSandbox(sandboxId: string): Promise<void> {
  return invoke("stop_sandbox", { sandboxId });
}

export async function destroySandbox(sandboxId: string): Promise<void> {
  return invoke("destroy_sandbox", { sandboxId });
}

export async function getSandboxForConversation(
  conversationId: string
): Promise<SandboxInfo | null> {
  return invoke("get_sandbox_for_conversation", { conversationId });
}
