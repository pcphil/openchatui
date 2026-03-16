import { create } from "zustand";
import type {
  SandboxInfo,
  ChangeProposal,
  SandboxEvent,
} from "../types/sandbox";
import * as sandboxApi from "../services/sandboxCommands";

interface OutputLine {
  stream: "stdout" | "stderr";
  text: string;
  timestamp: number;
}

interface SandboxState {
  activeSandbox: SandboxInfo | null;
  sandboxPanelOpen: boolean;
  outputLines: OutputLine[];
  pendingProposal: ChangeProposal | null;
  proposalHistory: Array<{
    proposal: ChangeProposal;
    approved: boolean;
    timestamp: number;
  }>;
  error: string | null;

  handleSandboxEvent: (event: SandboxEvent) => void;
  createSandbox: (
    conversationId: string,
    config?: Record<string, unknown>
  ) => Promise<void>;
  execInSandbox: (sandboxId: string, command: string[]) => Promise<number>;
  approveProposal: (proposalId: string) => Promise<void>;
  rejectProposal: (proposalId: string) => Promise<void>;
  stopSandbox: () => Promise<void>;
  destroySandbox: () => Promise<void>;
  openPanel: () => void;
  closePanel: () => void;
  clearOutput: () => void;
  clearError: () => void;
}

export const useSandboxStore = create<SandboxState>((set, get) => ({
  activeSandbox: null,
  sandboxPanelOpen: false,
  outputLines: [],
  pendingProposal: null,
  proposalHistory: [],
  error: null,

  handleSandboxEvent: (event: SandboxEvent) => {
    switch (event.event) {
      case "StatusChanged":
        set((state) => ({
          activeSandbox: state.activeSandbox
            ? { ...state.activeSandbox, status: event.data }
            : null,
        }));
        break;
      case "Output":
        set((state) => ({
          outputLines: [
            ...state.outputLines,
            {
              stream: event.data.stream as "stdout" | "stderr",
              text: event.data.text,
              timestamp: Date.now(),
            },
          ],
        }));
        break;
      case "ProposalReady":
        set({ pendingProposal: event.data });
        break;
      case "ProposalResult":
        set((state) => ({
          pendingProposal:
            state.pendingProposal?.proposal_id === event.data.proposal_id
              ? null
              : state.pendingProposal,
        }));
        break;
      case "Error":
        set({ error: event.data });
        break;
    }
  },

  createSandbox: async (conversationId, config) => {
    try {
      const { handleSandboxEvent } = get();
      const info = await sandboxApi.createSandbox(
        conversationId,
        config ?? null,
        handleSandboxEvent
      );
      set({
        activeSandbox: info,
        sandboxPanelOpen: true,
        outputLines: [],
        pendingProposal: null,
        proposalHistory: [],
        error: null,
      });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  execInSandbox: async (sandboxId, command) => {
    try {
      const { handleSandboxEvent } = get();
      return await sandboxApi.execInSandbox(
        sandboxId,
        command,
        handleSandboxEvent
      );
    } catch (e) {
      set({ error: String(e) });
      return -1;
    }
  },

  approveProposal: async (proposalId) => {
    try {
      const { handleSandboxEvent, pendingProposal } = get();
      if (pendingProposal && pendingProposal.proposal_id === proposalId) {
        set((state) => ({
          proposalHistory: [
            ...state.proposalHistory,
            {
              proposal: pendingProposal,
              approved: true,
              timestamp: Date.now(),
            },
          ],
          pendingProposal: null,
        }));
      }
      await sandboxApi.approveProposal(proposalId, handleSandboxEvent);
    } catch (e) {
      set({ error: String(e) });
    }
  },

  rejectProposal: async (proposalId) => {
    try {
      const { handleSandboxEvent, pendingProposal } = get();
      if (pendingProposal && pendingProposal.proposal_id === proposalId) {
        set((state) => ({
          proposalHistory: [
            ...state.proposalHistory,
            {
              proposal: pendingProposal,
              approved: false,
              timestamp: Date.now(),
            },
          ],
          pendingProposal: null,
        }));
      }
      await sandboxApi.rejectProposal(proposalId, handleSandboxEvent);
    } catch (e) {
      set({ error: String(e) });
    }
  },

  stopSandbox: async () => {
    const { activeSandbox } = get();
    if (!activeSandbox) return;
    try {
      await sandboxApi.stopSandbox(activeSandbox.sandbox_id);
      set((state) => ({
        activeSandbox: state.activeSandbox
          ? { ...state.activeSandbox, status: "stopped" as const }
          : null,
      }));
    } catch (e) {
      set({ error: String(e) });
    }
  },

  destroySandbox: async () => {
    const { activeSandbox } = get();
    if (!activeSandbox) return;
    try {
      await sandboxApi.destroySandbox(activeSandbox.sandbox_id);
      set({
        activeSandbox: null,
        sandboxPanelOpen: false,
        outputLines: [],
        pendingProposal: null,
        error: null,
      });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  openPanel: () => set({ sandboxPanelOpen: true }),
  closePanel: () => set({ sandboxPanelOpen: false }),
  clearOutput: () => set({ outputLines: [] }),
  clearError: () => set({ error: null }),
}));
