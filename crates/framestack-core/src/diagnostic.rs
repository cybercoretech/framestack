//! Diagnostics reported while parsing and validating a manifest.

use std::fmt;

use serde::Serialize;

/// How serious a diagnostic is. Any `Error` makes a manifest invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
}

/// Stable diagnostic codes. The code, not the message text, is the contract
/// that tests and other tools rely on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Code {
    /// TOML syntax error, unknown field, missing field, or wrong type.
    #[serde(rename = "FS001")]
    Parse,
    /// `manifest_version` is not a version this build understands.
    #[serde(rename = "FS002")]
    UnsupportedVersion,
    /// A component ID does not match the ID format.
    #[serde(rename = "FS003")]
    InvalidId,
    /// Two components share an ID.
    #[serde(rename = "FS004")]
    DuplicateId,
    /// A relationship endpoint names a component that does not exist.
    #[serde(rename = "FS005")]
    UnknownComponent,
    /// A relationship points from a component to itself.
    #[serde(rename = "FS006")]
    SelfRelationship,
    /// The `build_dependency` graph contains a cycle.
    #[serde(rename = "FS007")]
    BuildCycle,
    /// A capability identifier is not in the vocabulary.
    #[serde(rename = "FS008")]
    UnknownCapability,
    /// A required capability is not provided where the component can reach it.
    #[serde(rename = "FS009")]
    UnsatisfiedRequirement,
    /// A component must declare exactly one of `path` or `external`.
    #[serde(rename = "FS010")]
    Location,
    /// A component `path` is absolute or leaves the project directory.
    #[serde(rename = "FS011")]
    InvalidPath,
    /// A `config` table is keyed by an integration the component does not use.
    #[serde(rename = "FS012")]
    UnknownConfigNamespace,
    /// An environment variable reference is not a valid variable name.
    #[serde(rename = "FS013")]
    InvalidEnvName,
    /// The same relationship is declared more than once (warning).
    #[serde(rename = "FS014")]
    DuplicateRelationship,
    /// The `runtime_dependency` graph contains a cycle (warning, ADR-0005).
    #[serde(rename = "FS015")]
    RuntimeCycle,
    /// An integration identifier does not match the identifier format.
    #[serde(rename = "FS016")]
    InvalidIntegrationId,
    /// The project name is empty.
    #[serde(rename = "FS017")]
    EmptyProjectName,
    /// The same entry appears twice in one list (warning).
    #[serde(rename = "FS018")]
    DuplicateEntry,
}

impl Code {
    /// The code as it appears in output, for example `FS007`.
    pub fn as_str(self) -> &'static str {
        match self {
            Code::Parse => "FS001",
            Code::UnsupportedVersion => "FS002",
            Code::InvalidId => "FS003",
            Code::DuplicateId => "FS004",
            Code::UnknownComponent => "FS005",
            Code::SelfRelationship => "FS006",
            Code::BuildCycle => "FS007",
            Code::UnknownCapability => "FS008",
            Code::UnsatisfiedRequirement => "FS009",
            Code::Location => "FS010",
            Code::InvalidPath => "FS011",
            Code::UnknownConfigNamespace => "FS012",
            Code::InvalidEnvName => "FS013",
            Code::DuplicateRelationship => "FS014",
            Code::RuntimeCycle => "FS015",
            Code::InvalidIntegrationId => "FS016",
            Code::EmptyProjectName => "FS017",
            Code::DuplicateEntry => "FS018",
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where in the source text a diagnostic applies, when it is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Location {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number, counted in characters.
    pub column: usize,
}

/// One problem found in a manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    pub code: Code,
    pub severity: Severity,
    /// The manifest field the diagnostic is about, such as
    /// `components[1].requires[0]`. Empty for whole-document problems.
    pub path: String,
    pub message: String,
    /// Source position, known for parse errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

impl Diagnostic {
    pub fn error(code: Code, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Error,
            path: path.into(),
            message: message.into(),
            location: None,
        }
    }

    pub fn warning(code: Code, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            ..Self::error(code, path, message)
        }
    }

    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        write!(f, "{severity}[{}]", self.code)?;
        if !self.path.is_empty() {
            write!(f, " {}", self.path)?;
        }
        write!(f, ": {}", self.message)
    }
}
