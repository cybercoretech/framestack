//! What the Leptos + Axum adapter declares and plans.

use framestack_core::{Adapter, Code, Effect, Operation, parse};
use framestack_leptos_axum::{ADAPTER_ID, LeptosAxum};

fn component(extra: &str) -> framestack_core::Component {
    let manifest = parse(&format!(
        "manifest_version = 1\n[project]\nname = \"p\"\n\
         [[components]]\nid = \"site\"\nkind = \"application\"\n{extra}\nintegrations = [\"leptos\", \"axum\"]\n"
    ))
    .expect("test manifest parses");
    manifest
        .components
        .into_iter()
        .next()
        .expect("one component")
}

#[test]
fn describe_declares_handles_provides_and_tools() {
    let descriptor = LeptosAxum.describe();

    assert_eq!(descriptor.id, ADAPTER_ID);
    assert_eq!(ADAPTER_ID, "leptos-axum");
    assert_eq!(descriptor.handles, ["axum", "leptos"]);
    assert_eq!(
        descriptor.provides,
        ["frontend.ssr", "frontend.hydration", "http.server"]
    );
    assert_eq!(
        descriptor.operations,
        [Operation::Build, Operation::DevStart]
    );
    assert_eq!(descriptor.tools.len(), 1);
    assert_eq!(descriptor.tools[0].program, "cargo");
    assert_eq!(descriptor.tools[0].version_args, ["leptos", "--version"]);
    assert_eq!(
        descriptor.tools[0].install_hint,
        "cargo install cargo-leptos --locked --version 0.3.12"
    );
}

#[test]
fn build_plans_cargo_leptos_build_release() {
    let steps = LeptosAxum
        .plan(Operation::Build, &component("path = \"apps/site\""))
        .expect("plans");

    assert_eq!(steps.len(), 1);
    let step = &steps[0];
    assert_eq!(step.id, "site.build");
    assert_eq!(step.component, "site");
    assert_eq!(step.program, "cargo");
    assert_eq!(step.args, ["leptos", "build", "--release"]);
    assert_eq!(step.cwd, "apps/site");
    assert_eq!(
        step.effects,
        [
            Effect::WritesDir("target".into()),
            Effect::Network("crates.io".into())
        ]
    );
    assert!(!step.long_running);
}

#[test]
fn dev_plans_cargo_leptos_watch() {
    let steps = LeptosAxum
        .plan(Operation::DevStart, &component("path = \".\""))
        .expect("plans");

    assert_eq!(steps.len(), 1);
    let step = &steps[0];
    assert_eq!(step.id, "site.dev");
    assert_eq!(step.args, ["leptos", "watch"]);
    assert_eq!(step.cwd, ".");
    assert_eq!(step.env, ["LEPTOS_SITE_ADDR"]);
    assert!(step.long_running);
    assert!(
        step.effects
            .contains(&Effect::ListensOn("127.0.0.1:3000".into())),
        "{:?}",
        step.effects
    );
}

#[test]
fn settings_are_rejected() {
    let errors = LeptosAxum.validate(&component(
        "path = \".\"\n[components.config.leptos]\nanything = 1",
    ));

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, Code::UnsupportedSetting);
    assert_eq!(errors[0].path, "config.leptos");
}

#[test]
fn a_component_without_settings_is_valid() {
    assert!(LeptosAxum.validate(&component("path = \".\"")).is_empty());
}

#[test]
fn external_component_cannot_be_planned() {
    let error = LeptosAxum
        .plan(
            Operation::Build,
            &component("external = \"https://example.com\""),
        )
        .expect_err("needs a path");

    assert_eq!(error.code, Code::UnsupportedOperation);
    assert!(error.message.contains("path"), "{}", error.message);
}
