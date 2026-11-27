//! Build ordering from `build_dependency` relationships (ADR-0005).

use framestack_core::{graph, parse};

fn manifest(relationships: &str) -> framestack_core::Manifest {
    parse(&format!(
        r#"
manifest_version = 1
[project]
name = "order"
[[components]]
id = "web"
kind = "application"
path = "web"
[[components]]
id = "api"
kind = "service"
path = "api"
[[components]]
id = "contracts"
kind = "application"
path = "contracts"
{relationships}
"#
    ))
    .expect("test manifest parses")
}

#[test]
fn dependencies_are_built_before_their_dependents() {
    let m = manifest(
        r#"
[[relationships]]
from = "web"
to = "api"
type = "build_dependency"
[[relationships]]
from = "api"
to = "contracts"
type = "build_dependency"
"#,
    );

    assert_eq!(graph::build_order(&m).unwrap(), ["contracts", "api", "web"]);
}

#[test]
fn independent_components_keep_manifest_order() {
    let m = manifest("");

    assert_eq!(graph::build_order(&m).unwrap(), ["web", "api", "contracts"]);
}

#[test]
fn runtime_and_communication_edges_do_not_change_build_order() {
    let m = manifest(
        r#"
[[relationships]]
from = "web"
to = "contracts"
type = "runtime_dependency"
[[relationships]]
from = "web"
to = "api"
type = "communicates_with"
"#,
    );

    assert_eq!(graph::build_order(&m).unwrap(), ["web", "api", "contracts"]);
}

#[test]
fn there_is_no_build_order_when_builds_form_a_cycle() {
    let m = manifest(
        r#"
[[relationships]]
from = "web"
to = "api"
type = "build_dependency"
[[relationships]]
from = "api"
to = "web"
type = "build_dependency"
"#,
    );

    assert_eq!(graph::build_order(&m), None);
}
