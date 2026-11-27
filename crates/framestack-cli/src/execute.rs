//! Runs an approved plan: each step's program with its arguments, never a
//! shell, in the step's directory inside the project (ADR-0006).

use std::path::Path;
use std::process::{Command, ExitStatus};
use std::time::{Duration, Instant};

use framestack_core::{Code, Diagnostic, Plan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOutcome {
    Succeeded,
    /// The program ran and failed. `code` is `None` when a signal ended it.
    Failed {
        code: Option<i32>,
        signal: bool,
    },
    NotStarted {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct StepResult {
    pub id: String,
    pub outcome: StepOutcome,
    pub duration: Duration,
}

/// Runs the steps in order with the terminal attached, stopping after the
/// first step that does not succeed.
pub fn run(root: &Path, plan: &Plan) -> Vec<StepResult> {
    let mut results = Vec::with_capacity(plan.steps.len());
    for step in &plan.steps {
        let directory = root.join(&step.cwd);
        println!(
            "==> {}: {} {} (in {})",
            step.id,
            step.program,
            step.args.join(" "),
            directory.display()
        );
        let started = Instant::now();
        let outcome = match Command::new(&step.program)
            .args(&step.args)
            .current_dir(&directory)
            .status()
        {
            Ok(status) if status.success() => StepOutcome::Succeeded,
            Ok(status) => StepOutcome::Failed {
                code: status.code(),
                signal: ended_by_signal(status),
            },
            Err(error) => StepOutcome::NotStarted {
                reason: error.to_string(),
            },
        };
        let stop = outcome != StepOutcome::Succeeded;
        results.push(StepResult {
            id: step.id.clone(),
            outcome,
            duration: started.elapsed(),
        });
        if stop {
            break;
        }
    }
    results
}

#[cfg(unix)]
fn ended_by_signal(status: ExitStatus) -> bool {
    use std::os::unix::process::ExitStatusExt;
    status.signal().is_some()
}

#[cfg(not(unix))]
fn ended_by_signal(_status: ExitStatus) -> bool {
    false
}

/// FS022 for the step that did not succeed, if any.
pub fn failures(results: &[StepResult]) -> Vec<Diagnostic> {
    results
        .iter()
        .filter_map(|result| {
            let what = match &result.outcome {
                StepOutcome::Succeeded => return None,
                StepOutcome::Failed { signal: true, .. } => "was terminated by signal".to_owned(),
                StepOutcome::Failed {
                    code: Some(code), ..
                } => {
                    format!("failed with exit code {code}")
                }
                StepOutcome::Failed { code: None, .. } => "failed".to_owned(),
                StepOutcome::NotStarted { reason } => format!("could not start: {reason}"),
            };
            Some(Diagnostic::error(
                Code::StepFailed,
                format!("steps.{}", result.id),
                format!("step `{}` {what}", result.id),
            ))
        })
        .collect()
}
