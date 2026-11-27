//! Text, JSON and Mermaid output for the CLI.

use std::fmt::Write as _;
use std::path::Path;

use framestack_core::{Manifest, Report, Severity, graph};
use serde_json::json;

/// `1 component`, `2 components`.
fn count(n: usize, noun: &str) -> String {
    if n == 1 {
        format!("{n} {noun}")
    } else {
        format!("{n} {noun}s")
    }
}

/// Human-readable check result: each diagnostic, then a summary line.
pub fn check_text(path: &Path, report: &Report) -> String {
    let mut out = String::new();
    let shown = path.display();
    for diagnostic in &report.diagnostics {
        let _ = writeln!(out, "{diagnostic}");
        if let Some(at) = diagnostic.location {
            let _ = writeln!(out, "  --> {shown}:{}:{}", at.line, at.column);
        }
    }
    let errors = report.diagnostics.iter().filter(|d| d.is_error()).count();
    let warnings = report
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();

    match &report.manifest {
        Some(manifest) if report.is_valid() => {
            let _ = writeln!(
                out,
                "{shown}: valid: {}, {}",
                count(manifest.components.len(), "component"),
                count(manifest.relationships.len(), "relationship")
            );
        }
        _ => {}
    }
    if !report.diagnostics.is_empty() {
        let _ = writeln!(
            out,
            "{shown}: {}, {}",
            count(errors, "error"),
            count(warnings, "warning")
        );
    }
    out
}

/// Machine-readable check result.
pub fn check_json(path: &Path, report: &Report) -> serde_json::Result<String> {
    serde_json::to_string_pretty(&json!({
        "manifest": path.display().to_string(),
        "valid": report.is_valid(),
        "diagnostics": report.diagnostics,
    }))
}

/// Build order of a manifest that has already passed validation.
fn order(manifest: &Manifest) -> Vec<String> {
    // A valid manifest has no build cycle, so an order always exists.
    graph::build_order(manifest).unwrap_or_default()
}

pub fn graph_text(manifest: &Manifest) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{}: {}, {}",
        manifest.project.name,
        count(manifest.components.len(), "component"),
        count(manifest.relationships.len(), "relationship")
    );

    let _ = writeln!(out, "\ncomponents");
    for component in &manifest.components {
        let location = match (&component.path, &component.external) {
            (Some(path), _) => path.clone(),
            (None, Some(external)) => format!("external: {external}"),
            (None, None) => String::new(),
        };
        let _ = writeln!(
            out,
            "  {}  {}  {}",
            component.id,
            component.kind.as_str(),
            location
        );
    }

    let _ = writeln!(out, "\nrelationships");
    if manifest.relationships.is_empty() {
        let _ = writeln!(out, "  (none)");
    }
    for relationship in &manifest.relationships {
        let protocol = relationship
            .protocol
            .as_deref()
            .map(|p| format!(" ({p})"))
            .unwrap_or_default();
        let _ = writeln!(
            out,
            "  {} -> {}  {}{protocol}",
            relationship.from,
            relationship.to,
            relationship.kind.as_str()
        );
    }

    let _ = writeln!(out, "\nbuild order");
    for (position, id) in order(manifest).iter().enumerate() {
        let _ = writeln!(out, "  {}. {id}", position + 1);
    }
    out
}

pub fn graph_json(manifest: &Manifest) -> serde_json::Result<String> {
    let components: Vec<_> = manifest
        .components
        .iter()
        .map(|c| json!({ "id": c.id, "kind": c.kind.as_str() }))
        .collect();
    serde_json::to_string_pretty(&json!({
        "project": manifest.project.name,
        "components": components,
        "relationships": manifest.relationships,
        "build_order": order(manifest),
    }))
}

/// A Mermaid flowchart. Component IDs are already safe Mermaid node IDs.
pub fn graph_mermaid(manifest: &Manifest) -> String {
    let mut out = String::from("flowchart LR\n");
    for component in &manifest.components {
        let _ = writeln!(
            out,
            "  {id}[\"{id} ({kind})\"]",
            id = component.id,
            kind = component.kind.as_str()
        );
    }
    for relationship in &manifest.relationships {
        let _ = writeln!(
            out,
            "  {} -->|{}| {}",
            relationship.from,
            relationship.kind.as_str(),
            relationship.to
        );
    }
    out
}

/// The manifest `framestack init` writes.
pub fn starter_manifest(name: &str) -> serde_json::Result<String> {
    // A JSON string literal is also a valid TOML basic string.
    let quoted = serde_json::to_string(name)?;
    Ok(format!(
        r#"# FrameStack project manifest, format version 1.
# Check it with: framestack check
manifest_version = 1

[project]
name = {quoted}

# One entry per application, service, worker or resource.
[[components]]
id = "app"
kind = "application"
path = "."

# Relationships connect components, for example:
#
# [[relationships]]
# from = "web"
# to = "api"
# type = "communicates_with"   # or build_dependency, runtime_dependency
"#
    ))
}

/// Diagnostics, one per line, then an error and warning count.
pub fn diagnostics_text(diagnostics: &[framestack_core::Diagnostic]) -> String {
    let mut out = String::new();
    for diagnostic in diagnostics {
        let _ = writeln!(out, "{diagnostic}");
    }
    let errors = diagnostics.iter().filter(|d| d.is_error()).count();
    let _ = writeln!(
        out,
        "{}, {}",
        count(errors, "error"),
        count(diagnostics.len() - errors, "warning")
    );
    out
}

fn effect_text(effect: &framestack_core::Effect) -> String {
    use framestack_core::Effect;
    match effect {
        Effect::WritesDir(dir) => format!("writes {dir}"),
        Effect::Network(host) => format!("network {host}"),
        Effect::ListensOn(address) => format!("listens on {address}"),
    }
}

/// A plan as people read it: every step's command, directory and effects,
/// then the tool checks that were run.
pub fn plan_text(
    root: &Path,
    plan: &framestack_core::Plan,
    tools: &[crate::probe::ToolStatus],
) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "plan: {} ({})",
        plan.operation.as_str(),
        count(plan.steps.len(), "step")
    );
    if plan.steps.is_empty() {
        let _ = writeln!(out, "\nnothing to build");
    }
    for step in &plan.steps {
        let _ = writeln!(out, "\n{}  {}", step.id, step.description);
        let mut command = step.program.clone();
        for arg in &step.args {
            command.push(' ');
            command.push_str(arg);
        }
        let _ = writeln!(out, "  $ {command}");
        let _ = writeln!(out, "  in {}", root.join(&step.cwd).display());
        for name in &step.env {
            let _ = writeln!(out, "  reads {name}");
        }
        for effect in &step.effects {
            let _ = writeln!(out, "  {}", effect_text(effect));
        }
        if step.long_running {
            let _ = writeln!(out, "  runs until stopped");
        }
    }
    if !plan.skipped.is_empty() {
        let _ = writeln!(
            out,
            "\nnot built by FrameStack: {}",
            plan.skipped.join(", ")
        );
    }
    if !tools.is_empty() {
        let _ = writeln!(out, "\ntools");
        for tool in tools {
            let result = match (&tool.version, &tool.error) {
                (Some(version), _) => version.clone(),
                (None, Some(error)) => format!("failed: {error}"),
                (None, None) => String::new(),
            };
            let _ = writeln!(out, "  probe: {}  ->  {result}", tool.probe.join(" "));
        }
    }
    out
}

/// A plan for scripts: the plan itself, the project root and tool results.
pub fn plan_json(
    root: &Path,
    plan: &framestack_core::Plan,
    tools: &[crate::probe::ToolStatus],
) -> serde_json::Result<String> {
    serde_json::to_string_pretty(&json!({
        "root": root.display().to_string(),
        "operation": plan.operation,
        "steps": plan.steps,
        "skipped": plan.skipped,
        "tools": tools,
    }))
}

/// One line per step that ran: its ID, outcome and how long it took.
pub fn results_text(results: &[crate::execute::StepResult]) -> String {
    use crate::execute::StepOutcome;
    let mut out = String::new();
    for result in results {
        let outcome = match result.outcome {
            StepOutcome::Succeeded => "ok",
            StepOutcome::Failed { .. } => "failed",
            StepOutcome::NotStarted { .. } => "not started",
        };
        let _ = writeln!(
            out,
            "{}  {outcome}  ({:.1}s)",
            result.id,
            result.duration.as_secs_f64()
        );
    }
    out
}
