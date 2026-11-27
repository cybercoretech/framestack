//! What validation diagnostics tell the user: the field path and the
//! specific names involved.

use framestack_core::{Code, check};

fn diagnostic(source: &str, code: Code) -> framestack_core::Diagnostic {
    let report = check(source);
    report
        .diagnostics
        .into_iter()
        .find(|d| d.code == code)
        .unwrap_or_else(|| panic!("expected a {code} diagnostic"))
}

const HEADER: &str = "manifest_version = 1\n[project]\nname = \"x\"\n";

#[test]
fn build_cycle_message_lists_the_cycle_in_order() {
    let source = format!(
        "{HEADER}
[[components]]
id = \"web\"
kind = \"application\"
path = \"web\"
[[components]]
id = \"api\"
kind = \"service\"
path = \"api\"
[[components]]
id = \"shared\"
kind = \"service\"
path = \"shared\"
[[relationships]]
from = \"web\"
to = \"api\"
type = \"build_dependency\"
[[relationships]]
from = \"api\"
to = \"shared\"
type = \"build_dependency\"
[[relationships]]
from = \"shared\"
to = \"api\"
type = \"build_dependency\"
"
    );

    let found = diagnostic(&source, Code::BuildCycle);
    assert!(
        found.message.contains("api -> shared -> api"),
        "{}",
        found.message
    );
}

#[test]
fn duplicate_id_points_at_the_second_component() {
    let source = format!(
        "{HEADER}
[[components]]
id = \"web\"
kind = \"application\"
path = \"a\"
[[components]]
id = \"web\"
kind = \"application\"
path = \"b\"
"
    );

    assert_eq!(
        diagnostic(&source, Code::DuplicateId).path,
        "components[1].id"
    );
}

#[test]
fn unsatisfied_requirement_names_the_component_and_capability() {
    let source = format!(
        "{HEADER}
[[components]]
id = \"web\"
kind = \"application\"
path = \"web\"
requires = [\"api.rest\"]
"
    );

    let found = diagnostic(&source, Code::UnsatisfiedRequirement);
    assert_eq!(found.path, "components[0].requires[0]");
    assert!(found.message.contains("web"), "{}", found.message);
    assert!(found.message.contains("api.rest"), "{}", found.message);
}

#[test]
fn requirement_is_not_satisfied_through_a_build_dependency() {
    // A build dependency says nothing about runtime behaviour, so it cannot
    // supply a capability.
    let source = format!(
        "{HEADER}
[[components]]
id = \"web\"
kind = \"application\"
path = \"web\"
requires = [\"api.rest\"]
[[components]]
id = \"api\"
kind = \"service\"
path = \"api\"
provides = [\"api.rest\"]
[[relationships]]
from = \"web\"
to = \"api\"
type = \"build_dependency\"
"
    );

    assert_eq!(
        diagnostic(&source, Code::UnsatisfiedRequirement).path,
        "components[0].requires[0]"
    );
}

#[test]
fn requirement_is_satisfied_through_a_runtime_dependency() {
    let source = format!(
        "{HEADER}
[[components]]
id = \"api\"
kind = \"service\"
path = \"api\"
requires = [\"persistence.sql\"]
[[components]]
id = \"db\"
kind = \"resource\"
external = \"postgres://managed\"
provides = [\"persistence.sql\"]
[[relationships]]
from = \"api\"
to = \"db\"
type = \"runtime_dependency\"
"
    );

    let report = check(&source);
    assert!(report.diagnostics.is_empty(), "{:#?}", report.diagnostics);
}

#[test]
fn a_parse_error_returns_no_manifest() {
    let report = check("manifest_version = \"one\"\n");

    assert!(report.manifest.is_none());
    assert_eq!(report.diagnostics.len(), 1);
    assert_eq!(report.diagnostics[0].code, Code::Parse);
}
