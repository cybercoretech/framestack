//! `framestack build` and `framestack dev`: run the plan, only with --yes.
#![cfg(unix)]

mod support;

use std::fs;

use predicates::prelude::*;
use support::{Project, exec_lock};

#[test]
fn build_without_yes_prints_plan_and_exits_2() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("touch \"$MARKER\"");

    project
        .framestack()
        .env("MARKER", project.marker())
        .args(["build", "--manifest", project.root_str()])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("$ cargo leptos build --release"))
        .stderr(predicate::str::contains("--yes"));

    assert!(!project.marker().exists(), "nothing may run without --yes");
}

#[test]
fn build_yes_runs_the_step_in_the_component_directory() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("pwd -P > \"$MARKER\"");

    project
        .framestack()
        .env("MARKER", project.marker())
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("app.build  ok"));

    let ran_in = fs::read_to_string(project.marker()).unwrap();
    let expected = project.root().join("app").canonicalize().unwrap();
    assert_eq!(ran_in.trim(), expected.to_str().unwrap());
}

#[test]
fn failing_step_reports_fs022_and_exit_1() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("exit 3");

    project
        .framestack()
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("app.build  failed"))
        .stdout(predicate::str::contains("error[FS022]"))
        .stdout(predicate::str::contains("exit code 3"));
}

#[test]
fn unstartable_step_reports_fs022() {
    let _exec = exec_lock();
    // No app/ directory, so the step cannot start there.
    let project = Project::new(support::LEPTOS_APP);
    project.working_cargo("exit 0");

    project
        .framestack()
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("app.build  not started"))
        .stdout(predicate::str::contains("could not start"));
}

#[test]
fn signal_terminated_step_is_a_failure() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("kill -INT $$");

    project
        .framestack()
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("terminated by signal"));
}

#[test]
fn missing_tool_stops_before_any_step() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.fake_cargo(
        "if [ \"$1 $2\" = \"leptos --version\" ]; then exit 101; fi\ntouch \"$MARKER\"",
    );

    project
        .framestack()
        .env("MARKER", project.marker())
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS021]"));

    assert!(
        !project.marker().exists(),
        "no step may run after a failed tool check"
    );
}

#[test]
fn empty_plan_succeeds() {
    let _exec = exec_lock();
    let project = Project::new(
        "[[components]]\nid = \"db\"\nkind = \"resource\"\nexternal = \"postgres://managed\"\n",
    );

    project
        .framestack()
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("nothing to build"));
}

#[test]
fn dev_yes_runs_cargo_leptos_watch() {
    let _exec = exec_lock();
    let project = Project::leptos_app();
    project.working_cargo("echo \"$@\" > \"$MARKER\"");

    project
        .framestack()
        .env("MARKER", project.marker())
        .args(["dev", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(0);

    assert_eq!(
        fs::read_to_string(project.marker()).unwrap().trim(),
        "leptos watch"
    );
}

#[test]
fn build_of_an_invalid_manifest_exits_1_without_running() {
    let _exec = exec_lock();
    let project = Project::new("colour = \"blue\"\n");

    project
        .framestack()
        .args(["build", "--yes", "--manifest", project.root_str()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS001]"));
}
