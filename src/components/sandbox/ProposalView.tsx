import type { ChangeProposal } from "../../types/sandbox";
import { DiffViewer } from "./DiffViewer";

interface ProposalViewProps {
  proposal: ChangeProposal;
  onApprove: (proposalId: string) => void;
  onReject: (proposalId: string) => void;
}

export function ProposalView({
  proposal,
  onApprove,
  onReject,
}: ProposalViewProps) {
  return (
    <div
      className="flex flex-col gap-3 p-4 rounded-lg"
      style={{
        backgroundColor: "var(--bg-secondary)",
        border: "1px solid var(--accent, #3b82f6)",
      }}
    >
      <div className="flex items-center gap-2">
        <span
          className="text-xs font-semibold px-2 py-0.5 rounded"
          style={{
            backgroundColor: "var(--accent, #3b82f6)",
            color: "white",
          }}
        >
          PROPOSAL
        </span>
        <span
          className="text-sm font-medium"
          style={{ color: "var(--text-primary)" }}
        >
          {proposal.description}
        </span>
      </div>

      <div
        className="text-xs"
        style={{ color: "var(--text-secondary)" }}
      >
        {proposal.file_path}
      </div>

      <DiffViewer diff={proposal.diff} />

      <div className="flex gap-2 justify-end">
        <button
          onClick={() => onReject(proposal.proposal_id)}
          className="px-4 py-1.5 text-sm rounded font-medium transition-colors"
          style={{
            backgroundColor: "transparent",
            color: "var(--danger, #ef4444)",
            border: "1px solid var(--danger, #ef4444)",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = "rgba(239, 68, 68, 0.1)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          Reject
        </button>
        <button
          onClick={() => onApprove(proposal.proposal_id)}
          className="px-4 py-1.5 text-sm rounded font-medium transition-colors"
          style={{
            backgroundColor: "#22c55e",
            color: "white",
            border: "1px solid #22c55e",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = "#16a34a";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "#22c55e";
          }}
        >
          Approve
        </button>
      </div>
    </div>
  );
}
