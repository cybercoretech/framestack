//! Operation planning through adapters (ADR-0002, ADR-0006).
//!
//! The core decides which adapter owns each component, checks capabilities
//! and operations before anything runs, and collects the adapters' steps in
//! build order. Running the steps is not the core's job.

use serde::{Deserialize, Serialize};

use crate::diagnostic::{Code, Diagnostic};
use crate::graph;
use crate::manifest::{Component, Manifest};
use crate::validate::validate;

/// Operations FrameStack can ask an adapter to plan (ADR-0006 identifiers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "dev.start")]
    DevStart,
}

impl Operation {
    /// The ADR-0006 identifier, such as `dev.start`.
    pub fn as_str(self) -> &'static str {
        match self {
            Operation::Build => "build",
            Operation::DevStart => "dev.start",
        }
    }
}

/// An external program an adapter's steps need.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRequirement {
    /// Program to run, such as `cargo`.
    pub program: String,
    /// Arguments that make the program print its version, such as
    /// `["leptos", "--version"]`. Running them must have no other effect.
    pub version_args: Vec<String>,
    /// What to tell the user when the tool is missing.
    pub install_hint: String,
}

/// What an adapter is and what it can do (the ADR-0002 `describe` step).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrationDescriptor {
    /// Adapter ID, such as `leptos-axum`.
    pub id: String,
    pub version: String,
    /// The exact set of integration IDs whose lifecycle this adapter owns.
    pub handles: Vec<String>,
    /// Capabilities components handled by this adapter may declare.
    pub provides: Vec<String>,
    pub operations: Vec<Operation>,
    pub tools: Vec<ToolRequirement>,
}

/// Something a step does beyond running, declared so plans are reviewable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    /// Writes into this directory, relative to the step's `cwd`.
    WritesDir(String),
    /// Uses the network to reach this host.
    Network(String),
    /// Listens for connections on this address.
    ListensOn(String),
}

/// One program invocation. Never a shell string (ADR-0006).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    /// The component this step belongs to.
    pub component: String,
    pub description: String,
    pub program: String,
    pub args: Vec<String>,
    /// Working directory, relative to the project root.
    pub cwd: String,
    /// Names of environment variables the step reads.
    pub env: Vec<String>,
    pub effects: Vec<Effect>,
    /// True for steps that keep running until stopped, such as a dev server.
    pub long_running: bool,
}

/// Everything an operation would run, in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub operation: Operation,
    pub steps: Vec<Step>,
    /// Components FrameStack does not build, such as external resources.
    pub skipped: Vec<String>,
    /// Tools the steps need, each listed once in order of first use.
    pub tools: Vec<ToolRequirement>,
}

/// An in-process adapter (ADR-0002). Adapters plan; they never execute.
pub trait Adapter {
    fn describe(&self) -> IntegrationDescriptor;
    /// Adapter-specific checks of a component, such as its settings.
    fn validate(&self, component: &Component) -> Vec<Diagnostic>;
    /// The steps that perform `operation` for `component`.
    fn plan(&self, operation: Operation, component: &Component) -> Result<Vec<Step>, Diagnostic>;
}

/// Plans `operation` for every component of a valid manifest.
///
/// A component is handled by the adapter whose `handles` set equals the
/// component's integration set (order does not matter). Components with no
/// integrations are skipped. Capabilities and the operation are checked
/// before the adapter plans anything. Every problem found is returned.
pub fn plan_project(
    manifest: &Manifest,
    adapters: &[&dyn Adapter],
    operation: Operation,
) -> Result<Plan, Vec<Diagnostic>> {
    let invalid: Vec<Diagnostic> = validate(manifest)
        .into_iter()
        .filter(Diagnostic::is_error)
        .collect();
    if !invalid.is_empty() {
        return Err(invalid);
    }
    // A valid manifest has no build cycle, so an order exists.
    let order = graph::build_order(manifest).unwrap_or_default();

    let mut plan = Plan {
        operation,
        steps: Vec::new(),
        skipped: Vec::new(),
        tools: Vec::new(),
    };
    let mut errors = Vec::new();

    for id in &order {
        let Some((index, component)) = manifest
            .components
            .iter()
            .enumerate()
            .find(|(_, c)| &c.id == id)
        else {
            continue;
        };
        if component.integrations.is_empty() {
            plan.skipped.push(component.id.clone());
            continue;
        }
        let at = format!("components[{index}]");
        let Some((adapter, descriptor)) = find_adapter(adapters, component) else {
            errors.push(Diagnostic::error(
                Code::NoAdapter,
                format!("{at}.integrations"),
                format!(
                    "no adapter handles the integrations [{}] of component `{}`",
                    component.integrations.join(", "),
                    component.id
                ),
            ));
            continue;
        };

        let mut component_errors = Vec::new();
        for (position, capability) in component.provides.iter().enumerate() {
            if !descriptor.provides.contains(capability) {
                component_errors.push(Diagnostic::error(
                    Code::CapabilityNotProvided,
                    format!("{at}.provides[{position}]"),
                    format!(
                        "component `{}` declares `{capability}`, which adapter `{}` does not provide",
                        component.id, descriptor.id
                    ),
                ));
            }
        }
        if !descriptor.operations.contains(&operation) {
            component_errors.push(Diagnostic::error(
                Code::UnsupportedOperation,
                &at,
                format!(
                    "adapter `{}` cannot run `{}` for component `{}`",
                    descriptor.id,
                    operation.as_str(),
                    component.id
                ),
            ));
        }
        component_errors.extend(adapter.validate(component));
        if !component_errors.is_empty() {
            errors.extend(component_errors);
            continue;
        }

        match adapter.plan(operation, component) {
            Ok(steps) => {
                plan.steps.extend(steps);
                for tool in descriptor.tools {
                    if !plan.tools.iter().any(|t| t.program == tool.program) {
                        plan.tools.push(tool);
                    }
                }
            }
            Err(diagnostic) => errors.push(diagnostic),
        }
    }

    if errors.is_empty() {
        Ok(plan)
    } else {
        Err(errors)
    }
}

/// The adapter whose `handles` set equals the component's integrations.
fn find_adapter<'a>(
    adapters: &[&'a dyn Adapter],
    component: &Component,
) -> Option<(&'a dyn Adapter, IntegrationDescriptor)> {
    let mut wanted: Vec<&str> = component.integrations.iter().map(String::as_str).collect();
    wanted.sort_unstable();
    wanted.dedup();
    adapters.iter().find_map(|adapter| {
        let descriptor = adapter.describe();
        let mut handled: Vec<&str> = descriptor.handles.iter().map(String::as_str).collect();
        handled.sort_unstable();
        handled.dedup();
        (handled == wanted).then(|| (*adapter, descriptor.clone()))
    })
}
