import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeHighlight from "rehype-highlight";
import { Copy, Check } from "lucide-react";
import { useState, type ReactNode } from "react";

interface MarkdownRendererProps {
  content: string;
}

function CodeBlock({ children, className }: { children: ReactNode; className?: string }) {
  const [copied, setCopied] = useState(false);
  const isInline = !className;

  if (isInline) {
    return <code className={className}>{children}</code>;
  }

  const handleCopy = () => {
    const text = String(children).replace(/\n$/, "");
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const language = className?.replace("hljs language-", "").replace("language-", "") || "";

  return (
    <div className="relative group">
      <div className="flex items-center justify-between bg-[var(--bg-tertiary)] px-3 py-1.5 rounded-t-lg text-xs text-[var(--text-secondary)]">
        <span>{language}</span>
        <button
          onClick={handleCopy}
          className="flex items-center gap-1 hover:text-[var(--text-primary)] transition-colors"
        >
          {copied ? <Check size={13} /> : <Copy size={13} />}
          {copied ? "Copied" : "Copy"}
        </button>
      </div>
      <code className={className}>{children}</code>
    </div>
  );
}

export function MarkdownRenderer({ content }: MarkdownRendererProps) {
  return (
    <div className="markdown-content">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[rehypeHighlight]}
        components={{
          code({ children, className }) {
            return (
              <CodeBlock className={className}>
                {children}
              </CodeBlock>
            );
          },
        }}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}
