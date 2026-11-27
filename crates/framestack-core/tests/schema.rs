//! The published JSON Schema accepts what the Rust core produces and rejects
//! structurally invalid documents (ADR-0008).

use std::fs;
use std::path::{Path, PathBuf};

use framestack_core::parse;
use serde_json::{Value, json};

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn validator() -> jsonschema::Validator {
    let text = fs::read_to_string(repo_path("schemas/framestack-manifest-v1.schema.json"))
        .expect("schema file exists");
    let schema: Value = serde_json::from_str(&text).expect("schema is JSON");
    assert_eq!(
        schema["$schema"], "https://json-schema.org/draft/2020-12/schema",
        "ADR-0008 selects Draft 2020-12"
    );
    jsonschema::validator_for(&schema).expect("schema compiles")
}

#[test]
fn normalized_valid_and_warning_fixtures_match_the_schema() {
    let validator = validator();
    for group in ["valid", "warning"] {
        for entry in fs::read_dir(repo_path("fixtures/manifest").join(group)).unwrap() {
            let path = entry.unwrap().path();
            let manifest = parse(&fs::read_to_string(&path).unwrap()).unwrap();
            let document = manifest.to_normalized_json().unwrap();
            let errors: Vec<String> = validator
                .iter_errors(&document)
                .map(|e| e.to_string())
                .collect();
            assert!(errors.is_empty(), "{}: {errors:#?}", path.display());
        }
    }
}

fn minimal() -> Value {
    json!({
        "manifest_version": 1,
        "project": { "name": "x" },
        "components": [{
            "id": "web", "kind": "application", "path": "web",
            "integrations": [], "provides": [], "requires": [], "env": [], "config": {}
        }],
        "relationships": []
    })
}

#[test]
fn the_minimal_document_is_accepted() {
    assert!(validator().is_valid(&minimal()));
}

#[test]
fn an_unknown_top_level_field_is_rejected() {
    let mut document = minimal();
    document["colour"] = json!("blue");
    assert!(!validator().is_valid(&document));
}

#[test]
fn an_unknown_component_field_is_rejected() {
    let mut document = minimal();
    document["components"][0]["pathh"] = json!("web");
    assert!(!validator().is_valid(&document));
}

#[test]
fn an_unknown_component_kind_is_rejected() {
    let mut document = minimal();
    document["components"][0]["kind"] = json!("frontend");
    assert!(!validator().is_valid(&document));
}

#[test]
fn an_unknown_relationship_type_is_rejected() {
    let mut document = minimal();
    document["relationships"] = json!([{ "from": "web", "to": "web", "type": "depends_on" }]);
    assert!(!validator().is_valid(&document));
}

#[test]
fn a_missing_manifest_version_is_rejected() {
    let mut document = minimal();
    document.as_object_mut().unwrap().remove("manifest_version");
    assert!(!validator().is_valid(&document));
}
