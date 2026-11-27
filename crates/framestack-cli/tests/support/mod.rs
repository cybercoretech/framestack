//! Shared helpers for CLI tests that need a project on disk and a fake
//! `cargo` on `PATH`.

#![allow(dead_code)] // Each test file uses a different subset.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

use assert_cmd::Command;
use tempfile::TempDir;

/// Serializes writing fake programs with running them. When one test thread
/// starts a process while another still has a freshly written script open,
/// the child inherits the write handle and running the script fails with
/// "Text file busy" (ETXTBSY). Hold this for the whole test.
pub fn exec_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// A temporary project with a framestack.toml and, unless told otherwise,
/// the component's directory.
pub struct Project {
    pub dir: TempDir,
    pub bin: TempDir,
}

impl Project {
    pub fn new(components: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("framestack.toml"),
            format!("manifest_version = 1\n[project]\nname = \"ref\"\n{components}"),
        )
        .unwrap();
        Self {
            dir,
            bin: tempfile::tempdir().unwrap(),
        }
    }

    /// One Leptos + Axum application at `app/`, with its directory.
    pub fn leptos_app() -> Self {
        let project = Self::new(LEPTOS_APP);
        fs::create_dir(project.dir.path().join("app")).unwrap();
        project
    }

    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    pub fn root_str(&self) -> &str {
        self.dir.path().to_str().unwrap()
    }

    /// Writes an executable `cargo` script into this project's fake bin dir.
    pub fn fake_cargo(&self, body: &str) {
        let path = self.bin.path().join("cargo");
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).unwrap();
    }

    /// A fake `cargo` whose `leptos --version` succeeds and whose other
    /// invocations run `rest`.
    pub fn working_cargo(&self, rest: &str) {
        self.fake_cargo(&format!(
            "if [ \"$1 $2\" = \"leptos --version\" ]; then echo \"cargo-leptos 0.3.12\"; exit 0; fi\n{rest}"
        ));
    }

    /// `framestack` with PATH set to only this project's fake bin dir.
    pub fn framestack(&self) -> Command {
        let mut command = Command::cargo_bin("framestack").unwrap();
        command.env("PATH", self.bin.path());
        command
    }

    pub fn marker(&self) -> PathBuf {
        self.dir.path().join("marker")
    }
}

pub const LEPTOS_APP: &str = r#"
[[components]]
id = "app"
kind = "application"
path = "app"
integrations = ["leptos", "axum"]
provides = ["frontend.ssr", "frontend.hydration", "http.server"]
"#;
