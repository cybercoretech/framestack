//! FrameStack core: the project model, `framestack.toml` parsing, and
//! validation.

pub mod capability;
mod diagnostic;
pub mod graph;
mod manifest;
mod plan;
mod validate;

pub use diagnostic::{Code, Diagnostic, Location, Severity};
pub use manifest::{
    Component, ComponentKind, MANIFEST_FILE, Manifest, Project, Relationship, RelationshipType,
};
pub use plan::{
    Adapter, Effect, IntegrationDescriptor, Operation, Plan, Step, ToolRequirement, plan_project,
};
pub use validate::{SUPPORTED_VERSIONS, validate};

/// Parses `framestack.toml` source text into a [`Manifest`].
///
/// This checks structure only: syntax, required fields, types and unknown
/// fields. Use [`check`] for the full semantic validation.
pub fn parse(source: &str) -> Result<Manifest, Diagnostic> {
    toml::from_str(source).map_err(|error| {
        let mut diagnostic = Diagnostic::error(Code::Parse, "", error.message().trim());
        diagnostic.location = error.span().map(|span| location_of(source, span.start));
        diagnostic
    })
}

/// 1-based line and column of byte `offset` in `source`.
fn location_of(source: &str, offset: usize) -> Location {
    let before = &source[..offset.min(source.len())];
    let line = before.matches('\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    Location {
        line,
        column: before[line_start..].chars().count() + 1,
    }
}

/// The result of checking a manifest.
#[derive(Debug, Clone)]
pub struct Report {
    /// The parsed manifest, or `None` when it could not be parsed.
    pub manifest: Option<Manifest>,
    /// Every problem found, errors first in the order they were detected.
    pub diagnostics: Vec<Diagnostic>,
}

impl Report {
    /// True when no diagnostic is an error. Warnings do not make a manifest
    /// invalid.
    pub fn is_valid(&self) -> bool {
        self.manifest.is_some() && !self.diagnostics.iter().any(Diagnostic::is_error)
    }
}

/// Parses and fully validates `framestack.toml` source text.
pub fn check(source: &str) -> Report {
    match parse(source) {
        Ok(manifest) => {
            let mut diagnostics = validate(&manifest);
            // Errors first, keeping detection order within each severity.
            diagnostics.sort_by_key(|d| !d.is_error());
            Report {
                manifest: Some(manifest),
                diagnostics,
            }
        }
        Err(diagnostic) => Report {
            manifest: None,
            diagnostics: vec![diagnostic],
        },
    }
}
