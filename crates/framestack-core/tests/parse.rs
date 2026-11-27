//! Parsing `framestack.toml` into the manifest model (ADR-0008).

use framestack_core::{Code, ComponentKind, RelationshipType, parse};

const MINIMAL: &str = r#"
manifest_version = 1

[project]
name = "night-shift"

[[components]]
id = "web"
kind = "application"
path = "apps/web"
integrations = ["react", "vite"]
requires = ["http.api"]

[[components]]
id = "api"
kind = "service"
path = "services/api"
provides = ["http.api"]

[[relationships]]
from = "web"
to = "api"
type = "communicates_with"
protocol = "http"
"#;

#[test]
fn parses_components_and_relationships_in_manifest_order() {
    let manifest = parse(MINIMAL).expect("minimal manifest parses");

    assert_eq!(manifest.manifest_version, 1);
    assert_eq!(manifest.project.name, "night-shift");
    assert_eq!(manifest.components.len(), 2);
    assert_eq!(manifest.components[0].id, "web");
    assert_eq!(manifest.components[0].kind, ComponentKind::Application);
    assert_eq!(manifest.components[0].path.as_deref(), Some("apps/web"));
    assert_eq!(manifest.components[0].integrations, ["react", "vite"]);
    assert_eq!(manifest.components[0].requires, ["http.api"]);
    assert_eq!(manifest.components[1].kind, ComponentKind::Service);
    assert_eq!(manifest.components[1].provides, ["http.api"]);
    assert_eq!(manifest.relationships[0].from, "web");
    assert_eq!(manifest.relationships[0].to, "api");
    assert_eq!(
        manifest.relationships[0].kind,
        RelationshipType::CommunicatesWith
    );
    assert_eq!(manifest.relationships[0].protocol.as_deref(), Some("http"));
}

#[test]
fn optional_lists_default_to_empty() {
    let manifest = parse(
        r#"
manifest_version = 1
[project]
name = "bare"
"#,
    )
    .expect("manifest with no components parses");

    assert!(manifest.components.is_empty());
    assert!(manifest.relationships.is_empty());
}

#[test]
fn rejects_an_unknown_top_level_field_with_its_line() {
    let error = parse("manifest_version = 1\nfrobnicate = true\n[project]\nname = \"x\"\n")
        .expect_err("unknown field is rejected");

    assert_eq!(error.code, Code::Parse);
    assert!(error.message.contains("frobnicate"), "{}", error.message);
    assert_eq!(error.location.map(|l| l.line), Some(2));
}

#[test]
fn rejects_a_misspelled_component_field() {
    let error = parse(
        r#"
manifest_version = 1
[project]
name = "x"
[[components]]
id = "web"
kind = "application"
pathh = "web"
"#,
    )
    .expect_err("misspelled field is rejected");

    assert_eq!(error.code, Code::Parse);
    assert!(error.message.contains("pathh"), "{}", error.message);
}

#[test]
fn rejects_an_unknown_component_kind() {
    let error = parse(
        r#"
manifest_version = 1
[project]
name = "x"
[[components]]
id = "web"
kind = "frontend"
path = "web"
"#,
    )
    .expect_err("unknown kind is rejected");

    assert_eq!(error.code, Code::Parse);
    assert!(error.message.contains("frontend"), "{}", error.message);
}

#[test]
fn rejects_a_manifest_without_a_version() {
    let error = parse("[project]\nname = \"x\"\n").expect_err("version is required");

    assert_eq!(error.code, Code::Parse);
    assert!(
        error.message.contains("manifest_version"),
        "{}",
        error.message
    );
}

#[test]
fn rejects_invalid_toml_syntax_with_its_line() {
    let error = parse("manifest_version = 1\n[project\n").expect_err("bad syntax is rejected");

    assert_eq!(error.code, Code::Parse);
    assert_eq!(error.location.map(|l| l.line), Some(2));
}

#[test]
fn rejects_integration_config_that_is_not_a_table() {
    let error = parse(
        r#"
manifest_version = 1
[project]
name = "x"
[[components]]
id = "api"
kind = "service"
path = "api"
integrations = ["fastify"]
config.fastify = 5
"#,
    )
    .expect_err("config for an integration must be a table");

    assert_eq!(error.code, Code::Parse);
}
