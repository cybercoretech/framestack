//! Semantic validation of a parsed manifest (ADR-0005, ADR-0006, ADR-0008).
//!
//! JSON Schema covers structure; these rules cover what a schema cannot
//! express, such as references between components and graph cycles.

use std::collections::HashSet;
use std::path::{Component as PathPart, Path};

use crate::capability;
use crate::diagnostic::{Code, Diagnostic};
use crate::graph;
use crate::manifest::{Manifest, RelationshipType};

/// Manifest format versions this build can validate.
pub const SUPPORTED_VERSIONS: &[u32] = &[1];

/// Runs every semantic rule and returns the diagnostics in a stable order.
pub fn validate(manifest: &Manifest) -> Vec<Diagnostic> {
    let mut found = Vec::new();

    if !SUPPORTED_VERSIONS.contains(&manifest.manifest_version) {
        found.push(Diagnostic::error(
            Code::UnsupportedVersion,
            "manifest_version",
            format!(
                "manifest_version {} is not supported; this build reads version 1",
                manifest.manifest_version
            ),
        ));
        // The other rules are defined for version 1 only.
        return found;
    }

    if manifest.project.name.trim().is_empty() {
        found.push(Diagnostic::error(
            Code::EmptyProjectName,
            "project.name",
            "project name must not be empty",
        ));
    }

    check_components(manifest, &mut found);
    check_relationships(manifest, &mut found);
    check_requirements(manifest, &mut found);
    found
}

fn check_components(manifest: &Manifest, found: &mut Vec<Diagnostic>) {
    let mut seen = HashSet::new();
    for (index, component) in manifest.components.iter().enumerate() {
        let at = format!("components[{index}]");

        if !is_component_id(&component.id) {
            found.push(Diagnostic::error(
                Code::InvalidId,
                format!("{at}.id"),
                format!(
                    "`{}` is not a valid component ID; use lowercase letters, digits, `-` and `_`, starting with a letter",
                    component.id
                ),
            ));
        } else if !seen.insert(component.id.as_str()) {
            found.push(Diagnostic::error(
                Code::DuplicateId,
                format!("{at}.id"),
                format!("component ID `{}` is already used", component.id),
            ));
        }

        match (&component.path, &component.external) {
            (Some(path), None) => {
                if !is_project_relative(path) {
                    found.push(Diagnostic::error(
                        Code::InvalidPath,
                        format!("{at}.path"),
                        format!("`{path}` must be a relative path inside the project"),
                    ));
                }
            }
            (None, Some(_)) => {}
            (Some(_), Some(_)) => found.push(Diagnostic::error(
                Code::Location,
                &at,
                format!(
                    "component `{}` declares both `path` and `external`; use one",
                    component.id
                ),
            )),
            (None, None) => found.push(Diagnostic::error(
                Code::Location,
                &at,
                format!(
                    "component `{}` needs a `path` or an `external` location",
                    component.id
                ),
            )),
        }

        for (position, integration) in component.integrations.iter().enumerate() {
            if !is_integration_id(integration) {
                found.push(Diagnostic::error(
                    Code::InvalidIntegrationId,
                    format!("{at}.integrations[{position}]"),
                    format!("`{integration}` is not a valid integration ID"),
                ));
            }
        }

        for (field, list) in [
            ("provides", &component.provides),
            ("requires", &component.requires),
        ] {
            for (position, id) in list.iter().enumerate() {
                if !capability::is_known(id) {
                    found.push(Diagnostic::error(
                        Code::UnknownCapability,
                        format!("{at}.{field}[{position}]"),
                        format!("`{id}` is not a known capability"),
                    ));
                }
            }
        }

        for (position, name) in component.env.iter().enumerate() {
            if !is_env_name(name) {
                found.push(Diagnostic::error(
                    Code::InvalidEnvName,
                    format!("{at}.env[{position}]"),
                    format!(
                        "`{name}` is not a valid environment variable name; use A-Z, 0-9 and `_`"
                    ),
                ));
            }
        }

        for namespace in component.config.keys() {
            if !component.integrations.contains(namespace) {
                found.push(Diagnostic::error(
                    Code::UnknownConfigNamespace,
                    format!("{at}.config.{namespace}"),
                    format!(
                        "config for `{namespace}`, which component `{}` does not list in `integrations`",
                        component.id
                    ),
                ));
            }
        }

        for (field, list) in [
            ("integrations", &component.integrations),
            ("provides", &component.provides),
            ("requires", &component.requires),
            ("env", &component.env),
        ] {
            let mut entries = HashSet::new();
            for (position, entry) in list.iter().enumerate() {
                if !entries.insert(entry.as_str()) {
                    found.push(Diagnostic::warning(
                        Code::DuplicateEntry,
                        format!("{at}.{field}[{position}]"),
                        format!("`{entry}` is listed more than once"),
                    ));
                }
            }
        }
    }
}

fn check_relationships(manifest: &Manifest, found: &mut Vec<Diagnostic>) {
    let mut seen = HashSet::new();
    for (index, relationship) in manifest.relationships.iter().enumerate() {
        let at = format!("relationships[{index}]");
        let mut endpoints_exist = true;
        for (field, id) in [("from", &relationship.from), ("to", &relationship.to)] {
            if manifest.component(id).is_none() {
                endpoints_exist = false;
                found.push(Diagnostic::error(
                    Code::UnknownComponent,
                    format!("{at}.{field}"),
                    format!("no component has the ID `{id}`"),
                ));
            }
        }
        if !endpoints_exist {
            continue;
        }
        if relationship.from == relationship.to {
            found.push(Diagnostic::error(
                Code::SelfRelationship,
                &at,
                format!(
                    "component `{}` cannot have a relationship with itself",
                    relationship.from
                ),
            ));
        } else if !seen.insert((&relationship.from, &relationship.to, relationship.kind)) {
            found.push(Diagnostic::warning(
                Code::DuplicateRelationship,
                &at,
                format!(
                    "{} from `{}` to `{}` is declared more than once",
                    relationship.kind.as_str(),
                    relationship.from,
                    relationship.to
                ),
            ));
        }
    }

    if let Some(cycle) = graph::find_cycle(manifest, RelationshipType::BuildDependency) {
        found.push(Diagnostic::error(
            Code::BuildCycle,
            "relationships",
            format!("build dependencies form a cycle: {}", cycle.join(" -> ")),
        ));
    }
    if let Some(cycle) = graph::find_cycle(manifest, RelationshipType::RuntimeDependency) {
        found.push(Diagnostic::warning(
            Code::RuntimeCycle,
            "relationships",
            format!("runtime dependencies form a cycle: {}", cycle.join(" -> ")),
        ));
    }
}

/// A requirement is satisfied when the component provides the capability
/// itself, or a component it has a `runtime_dependency` or
/// `communicates_with` relationship to provides it. A build dependency does
/// not supply runtime behaviour, so it never satisfies a requirement.
fn check_requirements(manifest: &Manifest, found: &mut Vec<Diagnostic>) {
    for (index, component) in manifest.components.iter().enumerate() {
        for (position, needed) in component.requires.iter().enumerate() {
            if !capability::is_known(needed) {
                continue; // Already reported as an unknown capability.
            }
            let provided_by_self = component.provides.contains(needed);
            let provided_by_target = manifest
                .relationships
                .iter()
                .filter(|r| r.from == component.id && r.kind != RelationshipType::BuildDependency)
                .filter_map(|r| manifest.component(&r.to))
                .any(|target| target.provides.contains(needed));
            if !provided_by_self && !provided_by_target {
                found.push(Diagnostic::error(
                    Code::UnsatisfiedRequirement,
                    format!("components[{index}].requires[{position}]"),
                    format!(
                        "component `{}` requires `{needed}`, but neither it nor a component it depends on or communicates with provides it",
                        component.id
                    ),
                ));
            }
        }
    }
}

/// `web`, `api-gateway`, `worker_2`: a lowercase letter, then lowercase
/// letters and digits, with single `-` or `_` between them.
fn is_component_id(id: &str) -> bool {
    is_identifier(id, &['-', '_'])
}

/// `leptos`, `react-dom`, `vendor.plugin`: like a component ID, also
/// allowing `.` as a separator.
fn is_integration_id(id: &str) -> bool {
    is_identifier(id, &['-', '_', '.'])
}

fn is_identifier(id: &str, separators: &[char]) -> bool {
    let mut previous_was_separator = true;
    for (position, c) in id.chars().enumerate() {
        if separators.contains(&c) {
            if previous_was_separator {
                return false;
            }
            previous_was_separator = true;
        } else if c.is_ascii_lowercase() || (position > 0 && c.is_ascii_digit()) {
            previous_was_separator = false;
        } else {
            return false;
        }
    }
    !id.is_empty() && !previous_was_separator
}

/// `DATABASE_URL`, `PORT`: uppercase letters, digits and `_`, not starting
/// with a digit.
fn is_env_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_uppercase() || c == '_')
        && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

/// A relative path that stays inside the project: not empty, not absolute,
/// no `..`, and no backslashes (manifests must work on every OS).
fn is_project_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && Path::new(path)
            .components()
            .all(|part| matches!(part, PathPart::Normal(_) | PathPart::CurDir))
}
