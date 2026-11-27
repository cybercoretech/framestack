//! Runs the shared manifest fixtures in `fixtures/manifest/` (ADR-0008).
//!
//! Each fixture's first line states the expected outcome:
//! `# expect: valid`, `# expect: error FS007` or `# expect: warning FS015`.

use std::fs;
use std::path::{Path, PathBuf};

use framestack_core::{Severity, check};

fn fixture_dir(group: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/manifest")
        .join(group)
}

/// Every `.toml` fixture in a group, sorted, with its source text.
fn fixtures(group: &str) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = fs::read_dir(fixture_dir(group))
        .unwrap_or_else(|error| panic!("read fixtures/manifest/{group}: {error}"))
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .map(|path| {
            let name = format!("{group}/{}", path.file_name().unwrap().to_string_lossy());
            (
                name,
                fs::read_to_string(&path).expect("fixture is readable"),
            )
        })
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no fixtures in {group}");
    files
}

/// The `# expect:` header: `None` for valid, or the severity and code.
fn expectation(name: &str, source: &str) -> Option<(Severity, String)> {
    let header = source
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("# expect: "))
        .unwrap_or_else(|| panic!("{name}: first line must be `# expect: ...`"));
    let mut words = header.split_whitespace();
    match (words.next(), words.next()) {
        (Some("valid"), None) => None,
        (Some("error"), Some(code)) => Some((Severity::Error, code.to_owned())),
        (Some("warning"), Some(code)) => Some((Severity::Warning, code.to_owned())),
        _ => panic!("{name}: unrecognized expectation `{header}`"),
    }
}

#[test]
fn valid_fixtures_have_no_diagnostics() {
    for (name, source) in fixtures("valid") {
        assert_eq!(expectation(&name, &source), None, "{name} header");
        let report = check(&source);
        assert!(
            report.diagnostics.is_empty(),
            "{name}: {:#?}",
            report.diagnostics
        );
        assert!(report.manifest.is_some(), "{name}: manifest returned");
    }
}

#[test]
fn invalid_fixtures_report_only_their_expected_error() {
    for (name, source) in fixtures("invalid") {
        let (severity, code) =
            expectation(&name, &source).expect("invalid fixture expects an error");
        assert_eq!(severity, Severity::Error, "{name} header");
        let report = check(&source);
        let errors: Vec<&str> = report
            .diagnostics
            .iter()
            .filter(|d| d.is_error())
            .map(|d| d.code.as_str())
            .collect();
        assert!(
            !errors.is_empty() && errors.iter().all(|found| *found == code),
            "{name}: expected only {code}, got {:#?}",
            report.diagnostics
        );
        assert!(!report.is_valid(), "{name}: must be invalid");
    }
}

#[test]
fn warning_fixtures_are_valid_and_report_their_warning() {
    for (name, source) in fixtures("warning") {
        let (severity, code) =
            expectation(&name, &source).expect("warning fixture expects a warning");
        assert_eq!(severity, Severity::Warning, "{name} header");
        let report = check(&source);
        assert!(report.is_valid(), "{name}: {:#?}", report.diagnostics);
        let codes: Vec<&str> = report.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, [code.as_str()], "{name}");
    }
}
