import { useState } from "react";
import { useSandboxStore } from "../../stores/sandboxStore";
import { SandboxTerminal } from "./SandboxTerminal";
import { SandboxStatusBar } from "./SandboxStatusBar";
import { ProposalView } from "./ProposalView";

type Tab = "terminal" | "proposals";

export function SandboxPanel() {
  const [activeTab, setActiveTab] = useState<Tab>("terminal");

  const {
    activeSandbox,
    outputLines,
    pendingProposal,
    proposalHistory,
    stopSandbox,
    destroySandbox,
    closePanel,
    approveProposal,
    rejectProposal,
  } = useSandboxStore();

  if (!activeSandbox) return null;

  return (
    <div
      className="flex flex-col h-full"
      style={{
        backgroundColor: "var(--bg-primary)",
        borderLeft: "1px solid var(--border-color)",
      }}
    >
      <SandboxStatusBar
        status={activeSandbox.status}
        onStop={stopSandbox}
        onDestroy={destroySandbox}
        onClose={closePanel}
      />

      {/* Tab bar */}
      <div
        className="flex border-b"
        style={{ borderColor: "var(--border-color)" }}
      >
        {(["terminal", "proposals"] as Tab[]).map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className="px-4 py-2 text-xs font-medium transition-colors relative"
            style={{
              color:
                activeTab === tab
                  ? "var(--accent, #3b82f6)"
                  : "var(--text-secondary)",
              backgroundColor: "transparent",
            }}
          >
            {tab === "terminal" ? "Terminal" : "Proposals"}
            {tab === "proposals" && pendingProposal && (
              <span
                className="absolute top-1 right-1 w-2 h-2 rounded-full"
                style={{ backgroundColor: "var(--accent, #3b82f6)" }}
              />
            )}
            {activeTab === tab && (
              <div
                className="absolute bottom-0 left-0 right-0 h-0.5"
                style={{ backgroundColor: "var(--accent, #3b82f6)" }}
              />
            )}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="flex-1 overflow-hidden flex flex-col">
        {activeTab === "terminal" && <SandboxTerminal lines={outputLines} />}

        {activeTab === "proposals" && (
          <div
            className="flex-1 overflow-y-auto p-4 flex flex-col gap-4"
            style={{ backgroundColor: "var(--bg-primary)" }}
          >
            {pendingProposal && (
              <ProposalView
                proposal={pendingProposal}
                onApprove={approveProposal}
                onReject={rejectProposal}
              />
            )}

            {proposalHistory.length > 0 && (
              <div className="flex flex-col gap-2">
                <div
                  className="text-xs font-semibold"
                  style={{ color: "var(--text-secondary)" }}
                >
                  History
                </div>
                {[...proposalHistory].reverse().map((entry, i) => (
                  <div
                    key={i}
                    className="flex items-center gap-2 px-3 py-2 rounded text-xs"
                    style={{
                      backgroundColor: "var(--bg-secondary)",
                      color: "var(--text-secondary)",
                    }}
                  >
                    <span
                      className="inline-block w-2 h-2 rounded-full"
                      style={{
                        backgroundColor: entry.approved
                          ? "#22c55e"
                          : "#ef4444",
                      }}
                    />
                    <span className="flex-1 truncate">
                      {entry.proposal.description}
                    </span>
                    <span>{entry.proposal.file_path}</span>
                  </div>
                ))}
              </div>
            )}

            {!pendingProposal && proposalHistory.length === 0 && (
              <div
                className="flex-1 flex items-center justify-center text-sm"
                style={{ color: "var(--text-secondary)" }}
              >
                No proposals yet
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
