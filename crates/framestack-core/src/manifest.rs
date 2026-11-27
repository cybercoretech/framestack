//! The `framestack.toml` data model (manifest format version 1).
//!
//! Core-defined tables reject unknown fields (ADR-0008). Integration-specific
//! settings live under `config.<integration>` and are preserved as-is for the
//! integration's adapter to validate.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The file name FrameStack looks for in a project directory.
pub const MANIFEST_FILE: &str = "framestack.toml";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Manifest format version. Required.
    pub manifest_version: u32,
    pub project: Project,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Structural kind of a component (ADR-0005). Framework identity comes from
/// integrations, not from the kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentKind {
    Application,
    Service,
    Worker,
    Resource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// Stable, project-local identifier.
    pub id: String,
    pub kind: ComponentKind,
    /// Working directory, relative to the manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Location of something FrameStack does not build, such as a managed
    /// database. Used instead of `path`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Integration identifiers, such as `leptos` or `fastify`.
    #[serde(default)]
    pub integrations: Vec<String>,
    /// Capabilities this component provides.
    #[serde(default)]
    pub provides: Vec<String>,
    /// Capabilities this component needs from itself or from a component it
    /// depends on or communicates with.
    #[serde(default)]
    pub requires: Vec<String>,
    /// Names of environment variables the component reads. Values never go
    /// in the manifest.
    #[serde(default)]
    pub env: Vec<String>,
    /// Integration-specific settings: one table per integration identifier.
    #[serde(default)]
    pub config: BTreeMap<String, serde_json::Map<String, serde_json::Value>>,
}

/// Relationship types from ADR-0005.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    /// The source needs the target built first.
    BuildDependency,
    /// The source needs the target available at runtime.
    RuntimeDependency,
    /// The source talks to the target through a declared interface.
    CommunicatesWith,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub kind: RelationshipType,
    /// Protocol for `communicates_with`, such as `http`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl ComponentKind {
    /// The name used in the manifest, such as `application`.
    pub fn as_str(self) -> &'static str {
        match self {
            ComponentKind::Application => "application",
            ComponentKind::Service => "service",
            ComponentKind::Worker => "worker",
            ComponentKind::Resource => "resource",
        }
    }
}

impl RelationshipType {
    /// The name used in the manifest, such as `build_dependency`.
    pub fn as_str(self) -> &'static str {
        match self {
            RelationshipType::BuildDependency => "build_dependency",
            RelationshipType::RuntimeDependency => "runtime_dependency",
            RelationshipType::CommunicatesWith => "communicates_with",
        }
    }
}

impl Manifest {
    /// The component with this ID, if there is one.
    pub fn component(&self, id: &str) -> Option<&Component> {
        self.components.iter().find(|component| component.id == id)
    }

    /// The normalized, language-neutral JSON form of the manifest. Every list
    /// and table is present, even when empty, so consumers need no defaults.
    pub fn to_normalized_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
