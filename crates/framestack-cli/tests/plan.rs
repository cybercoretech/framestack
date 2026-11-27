//! `framestack plan build|dev`: shows what would run, checks tools, runs
//! nothing else.
#![cfg(unix)]

mod support;

use predicates::prelude::*;
use serde_json::Value;
use support::{Project, exec_lock};

#[test]
fn plan_build_text_lists_step_command_and_effects() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("touch \"$FRAMESTACK_TEST_MARKER\"");

    project
        .framestack()
        .env("FRAMESTACK_TEST_MARKER", project.marker())
        .args(["plan", "build", "--manifest", project.root_str()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("$ cargo leptos build --release"))
        .stdout(predicate::str::contains("writes target"))
        .stdout(predicate::str::contains("network crates.io"))
        .stdout(predicate::str::contains("probe: cargo leptos --version"))
        .stdout(predicate::str::contains("cargo-leptos 0.3.12"));

    assert!(
        !project.marker().exists(),
        "plan must not run the build step"
    );
}

#[test]
fn plan_dev_shows_the_long_running_watch_step() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("exit 0");

    project
        .framestack()
        .args(["plan", "dev", "--manifest", project.root_str()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("$ cargo leptos watch"))
        .stdout(predicate::str::contains("listens on 127.0.0.1:3000"))
        .stdout(predicate::str::contains("reads LEPTOS_SITE_ADDR"))
        .stdout(predicate::str::contains("runs until stopped"));
}

#[test]
fn plan_json_has_operation_steps_and_tools() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("exit 0");

    let output = project
        .framestack()
        .args([
            "plan",
            "build",
            "--manifest",
            project.root_str(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    let plan: Value = serde_json::from_slice(&output.stdout).expect("JSON output");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(plan["operation"], "build");
    assert_eq!(
        plan["steps"][0]["args"],
        serde_json::json!(["leptos", "build", "--release"])
    );
    assert_eq!(plan["tools"][0]["program"], "cargo");
    assert_eq!(plan["tools"][0]["version"], "cargo-leptos 0.3.12");
}

#[test]
fn plan_resolves_cwd_from_the_manifest_directory() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("exit 0");
    let elsewhere = tempfile::tempdir().unwrap();

    project
        .framestack()
        .current_dir(elsewhere.path())
        .args(["plan", "build", "--manifest", project.root_str()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains(format!(
            "in {}",
            project.root().join("app").display()
        )));
}

#[test]
fn probe_failure_is_reported_as_missing_tool() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.fake_cargo("echo 'error: no such command: `leptos`' >&2\nexit 101");

    project
        .framestack()
        .args(["plan", "build", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS021]"))
        .stdout(predicate::str::contains("no such command"))
        .stdout(predicate::str::contains(
            "cargo install cargo-leptos --locked --version 0.3.12",
        ));
}

#[test]
fn missing_tool_is_reported() {
    let _exec = exec_lock();
    let project = Project::leptos_app();

    project
        .framestack()
        .args(["plan", "build", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS021]"))
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn plan_of_partial_integrations_reports_fs020() {
    let _exec = exec_lock();
    let project = Project::new(
        "[[components]]\nid = \"api\"\nkind = \"service\"\npath = \"api\"\nintegrations = [\"axum\"]\n",
    );

    project
        .framestack()
        .args(["plan", "build", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "error[FS020] components[0].integrations",
        ));
}

#[test]
fn plan_of_an_invalid_manifest_reports_the_check_errors() {
    let _exec = exec_lock();
    let project = Project::new("colour = \"blue\"\n");

    project
        .framestack()
        .args(["plan", "build", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS001]"));
}
