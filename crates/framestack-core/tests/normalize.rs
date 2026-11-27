//! The normalized, language-neutral JSON form of a manifest (ADR-0008).

use framestack_core::parse;
use serde_json::json;

#[test]
fn normalized_json_fills_every_list_and_omits_unset_options() {
    let manifest = parse(
        r#"
manifest_version = 1

[project]
name = "night-shift"

[[components]]
id = "db"
kind = "resource"
external = "postgres://managed"

[components.config.postgres]
version = 17
"#,
    )
    .expect("manifest parses");

    let expected = json!({
        "manifest_version": 1,
        "project": { "name": "night-shift" },
        "components": [{
            "id": "db",
            "kind": "resource",
            "external": "postgres://managed",
            "integrations": [],
            "provides": [],
            "requires": [],
            "env": [],
            "config": { "postgres": { "version": 17 } }
        }],
        "relationships": []
    });

    assert_eq!(manifest.to_normalized_json().expect("serializes"), expected);
}

#[test]
fn normalized_json_names_the_relationship_type_field_type() {
    let manifest = parse(
        r#"
manifest_version = 1
[project]
name = "x"
[[relationships]]
from = "a"
to = "b"
type = "build_dependency"
"#,
    )
    .expect("manifest parses");

    assert_eq!(
        manifest.to_normalized_json().expect("serializes")["relationships"],
        json!([{ "from": "a", "to": "b", "type": "build_dependency" }])
    );
}
