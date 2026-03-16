export interface SandboxConfig {
  project_dir?: string;
  memory_limit?: string;
  cpu_limit?: number;
  network_enabled?: boolean;
  environment?: Record<string, string>;
}

export type SandboxStatus =
  | "creating"
  | "running"
  | "stopped"
  | "failed"
  | "destroyed";

export interface SandboxInfo {
  sandbox_id: string;
  conversation_id: string;
  status: SandboxStatus;
  container_id: string | null;
  created_at: string;
}

export interface ChangeProposal {
  proposal_id: string;
  file_path: string;
  description: string;
  diff: string;
  original_content: string | null;
  proposed_content: string | null;
}

export type SandboxEvent =
  | { event: "StatusChanged"; data: SandboxStatus }
  | { event: "Output"; data: { stream: string; text: string } }
  | { event: "ProposalReady"; data: ChangeProposal }
  | { event: "ProposalResult"; data: { proposal_id: string; approved: boolean } }
  | { event: "Error"; data: string };
