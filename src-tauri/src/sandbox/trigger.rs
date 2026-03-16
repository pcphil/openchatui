use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTrigger {
    pub action: String,
    pub description: Option<String>,
    pub project_dir: Option<String>,
}

/// Parse a sandbox trigger block from an LLM response.
/// Looks for ```sandbox {...} ``` blocks.
pub fn parse_sandbox_trigger(response: &str) -> Option<SandboxTrigger> {
    let start_marker = "```sandbox";
    let end_marker = "```";

    let start_idx = response.find(start_marker)?;
    let content_start = start_idx + start_marker.len();
    let remaining = &response[content_start..];
    let end_idx = remaining.find(end_marker)?;
    let json_str = remaining[..end_idx].trim();

    serde_json::from_str(json_str).ok()
}

/// Strip sandbox trigger blocks from the response text so they don't appear in the chat.
pub fn strip_sandbox_block(response: &str) -> String {
    let start_marker = "```sandbox";
    let end_marker = "```";

    let mut result = response.to_string();

    while let Some(start_idx) = result.find(start_marker) {
        let content_start = start_idx + start_marker.len();
        if let Some(end_idx) = result[content_start..].find(end_marker) {
            let block_end = content_start + end_idx + end_marker.len();
            result = format!("{}{}", &result[..start_idx], &result[block_end..]);
        } else {
            break;
        }
    }

    result.trim().to_string()
}

/// System prompt addition that instructs the LLM to emit sandbox triggers.
pub const SANDBOX_SYSTEM_PROMPT: &str = r#"You have access to a coding sandbox environment. When the user asks you to write, fix, debug, or modify code in a project, you should trigger the sandbox by including a special block in your response:

```sandbox
{"action": "launch", "description": "Brief description of the coding task", "project_dir": "optional/path/to/project"}
```

Only use this when the user explicitly asks for code changes, debugging, or implementation work. Do not trigger the sandbox for questions about code, explanations, or general discussion."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sandbox_trigger() {
        let response = r#"I'll help you fix that bug. Let me set up a sandbox.

```sandbox
{"action": "launch", "description": "Fix null check in auth.ts", "project_dir": "C:/Users/Phil/projects/myapp"}
```

I'll work on this right away."#;

        let trigger = parse_sandbox_trigger(response).unwrap();
        assert_eq!(trigger.action, "launch");
        assert_eq!(
            trigger.description.unwrap(),
            "Fix null check in auth.ts"
        );
        assert_eq!(
            trigger.project_dir.unwrap(),
            "C:/Users/Phil/projects/myapp"
        );
    }

    #[test]
    fn test_strip_sandbox_block() {
        let response = r#"I'll help you fix that bug.

```sandbox
{"action": "launch", "description": "Fix null check"}
```

Working on it now."#;

        let stripped = strip_sandbox_block(response);
        assert_eq!(stripped, "I'll help you fix that bug.\n\nWorking on it now.");
    }

    #[test]
    fn test_no_trigger() {
        let response = "Just a normal response without any sandbox block.";
        assert!(parse_sandbox_trigger(response).is_none());
    }
}
