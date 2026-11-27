//! Planning operations through adapters (ADR-0002, ADR-0006).

use framestack_core::{
    Adapter, Code, Component, Diagnostic, IntegrationDescriptor, Operation, Step, parse,
    plan_project,
};

/// A test adapter that owns `axum` + `leptos`, provides `http.server`,
/// supports only `build`, and plans one step per component.
struct Fake {
    settings_error: bool,
}

impl Fake {
    fn new() -> Self {
        Self {
            settings_error: false,
        }
    }
}

impl Adapter for Fake {
    fn describe(&self) -> IntegrationDescriptor {
        IntegrationDescriptor {
            id: "fake".into(),
            version: "0.0.1".into(),
            handles: vec!["axum".into(), "leptos".into()],
            provides: vec!["http.server".into()],
            operations: vec![Operation::Build],
            tools: Vec::new(),
        }
    }

    fn validate(&self, _component: &Component) -> Vec<Diagnostic> {
        if self.settings_error {
            vec![Diagnostic::error(
                Code::UnsupportedSetting,
                "x",
                "no settings",
            )]
        } else {
            Vec::new()
        }
    }

    fn plan(&self, _operation: Operation, component: &Component) -> Result<Vec<Step>, Diagnostic> {
        Ok(vec![Step {
            id: format!("{}.build", component.id),
            component: component.id.clone(),
            description: "fake build".into(),
            program: "true".into(),
            args: Vec::new(),
            cwd: component.path.clone().unwrap_or_default(),
            env: Vec::new(),
            effects: Vec::new(),
            long_running: false,
        }])
    }
}

const HEADER: &str = "manifest_version = 1\n[project]\nname = \"p\"\n";

fn manifest(body: &str) -> framestack_core::Manifest {
    parse(&format!("{HEADER}{body}")).expect("test manifest parses")
}

fn codes(errors: &[Diagnostic]) -> Vec<&'static str> {
    errors.iter().map(|d| d.code.as_str()).collect()
}

#[test]
fn matches_adapter_by_exact_integration_set() {
    let m = manifest(
        "[[components]]\nid = \"app\"\nkind = \"application\"\npath = \"app\"\nintegrations = [\"leptos\", \"axum\"]\n",
    );

    let plan = plan_project(&m, &[&Fake::new()], Operation::Build).expect("plans");

    assert_eq!(plan.operation, Operation::Build);
    assert_eq!(plan.steps.len(), 1);
    assert_eq!(plan.steps[0].id, "app.build");
    assert_eq!(plan.steps[0].cwd, "app");
}

#[test]
fn partial_integration_set_has_no_adapter() {
    let m = manifest(
        "[[components]]\nid = \"api\"\nkind = \"service\"\npath = \"api\"\nintegrations = [\"axum\"]\n",
    );

    let errors = plan_project(&m, &[&Fake::new()], Operation::Build).expect_err("no adapter");

    assert_eq!(codes(&errors), ["FS020"]);
    assert_eq!(errors[0].path, "components[0].integrations");
}

#[test]
fn declared_capability_must_be_provided_by_adapter() {
    let m = manifest(
        "[[components]]\nid = \"app\"\nkind = \"application\"\npath = \"app\"\nintegrations = [\"axum\", \"leptos\"]\nprovides = [\"frontend.ssr\"]\n",
    );

    let errors = plan_project(&m, &[&Fake::new()], Operation::Build).expect_err("not provided");

    assert_eq!(codes(&errors), ["FS019"]);
    assert_eq!(errors[0].path, "components[0].provides[0]");
}

#[test]
fn unsupported_operation_is_reported() {
    let m = manifest(
        "[[components]]\nid = \"app\"\nkind = \"application\"\npath = \"app\"\nintegrations = [\"axum\", \"leptos\"]\n",
    );

    let errors = plan_project(&m, &[&Fake::new()], Operation::DevStart).expect_err("no dev");

    assert_eq!(codes(&errors), ["FS024"]);
}

#[test]
fn components_without_integrations_are_skipped() {
    let m = manifest(
        "[[components]]\nid = \"db\"\nkind = \"resource\"\nexternal = \"postgres://managed\"\n",
    );

    let plan = plan_project(&m, &[&Fake::new()], Operation::Build).expect("plans");

    assert!(plan.steps.is_empty());
    assert_eq!(plan.skipped, ["db"]);
}

#[test]
fn steps_follow_build_order() {
    let m = manifest(
        "[[components]]\nid = \"web\"\nkind = \"application\"\npath = \"web\"\nintegrations = [\"axum\", \"leptos\"]\n\
         [[components]]\nid = \"api\"\nkind = \"service\"\npath = \"api\"\nintegrations = [\"axum\", \"leptos\"]\n\
         [[relationships]]\nfrom = \"web\"\nto = \"api\"\ntype = \"build_dependency\"\n",
    );

    let plan = plan_project(&m, &[&Fake::new()], Operation::Build).expect("plans");
    let order: Vec<&str> = plan.steps.iter().map(|s| s.component.as_str()).collect();

    assert_eq!(order, ["api", "web"]);
}

#[test]
fn adapter_validation_errors_stop_planning() {
    let m = manifest(
        "[[components]]\nid = \"app\"\nkind = \"application\"\npath = \"app\"\nintegrations = [\"axum\", \"leptos\"]\n",
    );
    let adapter = Fake {
        settings_error: true,
    };

    let errors = plan_project(&m, &[&adapter], Operation::Build).expect_err("invalid settings");

    assert_eq!(codes(&errors), ["FS023"]);
}

#[test]
fn invalid_manifest_is_not_planned() {
    let m = manifest(
        "[[components]]\nid = \"app\"\nkind = \"application\"\npath = \"a\"\n\
         [[components]]\nid = \"app\"\nkind = \"application\"\npath = \"b\"\n",
    );

    let errors = plan_project(&m, &[&Fake::new()], Operation::Build).expect_err("invalid");

    assert!(codes(&errors).contains(&"FS004"), "{errors:#?}");
}

#[test]
fn plan_serializes_operation_and_effect_names() {
    let json = serde_json::to_value(Operation::DevStart).unwrap();
    assert_eq!(json, "dev.start");
    let effect = serde_json::to_value(framestack_core::Effect::WritesDir("target".into())).unwrap();
    assert_eq!(effect, serde_json::json!({ "writes_dir": "target" }));
}
