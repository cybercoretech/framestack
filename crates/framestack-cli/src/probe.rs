//! Tool checks: run each declared version command and record the answer.

use std::io::ErrorKind;
use std::process::{Command, Stdio};

use framestack_core::{Code, Diagnostic, ToolRequirement};
use serde::Serialize;

/// The result of running one tool's version command.
#[derive(Debug, Clone, Serialize)]
pub struct ToolStatus {
    pub program: String,
    /// The full command that was run, such as `["cargo", "leptos", "--version"]`.
    pub probe: Vec<String>,
    /// First line the tool printed, when it ran successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Why the check failed, when it did.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip)]
    pub install_hint: String,
}

/// Runs every tool's version command, with no input and captured output.
pub fn probe(tools: &[ToolRequirement]) -> Vec<ToolStatus> {
    tools
        .iter()
        .map(|tool| {
            let mut probe = vec![tool.program.clone()];
            probe.extend(tool.version_args.iter().cloned());
            let result = Command::new(&tool.program)
                .args(&tool.version_args)
                .stdin(Stdio::null())
                .output();
            let (version, error) = match result {
                Ok(output) if output.status.success() => (Some(first_line(&output.stdout)), None),
                Ok(output) => {
                    let code = output
                        .status
                        .code()
                        .map_or_else(|| "a signal".to_owned(), |c| format!("exit {c}"));
                    let said = first_line(&output.stderr);
                    let reason = if said.is_empty() {
                        code
                    } else {
                        format!("{code}: {said}")
                    };
                    (None, Some(reason))
                }
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    (None, Some(format!("`{}` not found on PATH", tool.program)))
                }
                Err(error) => (None, Some(error.to_string())),
            };
            ToolStatus {
                program: tool.program.clone(),
                probe,
                version,
                error,
                install_hint: tool.install_hint.clone(),
            }
        })
        .collect()
}

/// FS021 for every tool whose check failed.
pub fn missing(statuses: &[ToolStatus]) -> Vec<Diagnostic> {
    statuses
        .iter()
        .filter_map(|status| {
            status.error.as_ref().map(|error| {
                Diagnostic::error(
                    Code::ToolMissing,
                    format!("tools.{}", status.program),
                    format!(
                        "`{}` failed: {error}; install with: {}",
                        status.probe.join(" "),
                        status.install_hint
                    ),
                )
            })
        })
        .collect()
}

fn first_line(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .to_owned()
}
