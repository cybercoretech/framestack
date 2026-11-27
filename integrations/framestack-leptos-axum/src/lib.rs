//! FrameStack adapter for Leptos + Axum applications built with cargo-leptos.
//!
//! The adapter only plans commands. It does not depend on Leptos or Axum:
//! the application's own Cargo project and cargo-leptos do the building.

use framestack_core::{
    Adapter, Code, Component, Diagnostic, Effect, IntegrationDescriptor, Operation, Step,
    ToolRequirement,
};

/// The adapter's ID in plans and diagnostics.
pub const ADAPTER_ID: &str = "leptos-axum";

/// The cargo-leptos release the adapter is tested against.
const CARGO_LEPTOS_VERSION: &str = "0.3.12";

/// Where `cargo leptos watch` serves by default, unless `LEPTOS_SITE_ADDR`
/// says otherwise.
const DEFAULT_SITE_ADDR: &str = "127.0.0.1:3000";

/// Plans cargo-leptos builds for components with
/// `integrations = ["leptos", "axum"]`.
pub struct LeptosAxum;

impl Adapter for LeptosAxum {
    fn describe(&self) -> IntegrationDescriptor {
        IntegrationDescriptor {
            id: ADAPTER_ID.to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            handles: vec!["axum".to_owned(), "leptos".to_owned()],
            provides: vec![
                "frontend.ssr".to_owned(),
                "frontend.hydration".to_owned(),
                "http.server".to_owned(),
            ],
            operations: vec![Operation::Build, Operation::DevStart],
            tools: vec![ToolRequirement {
                program: "cargo".to_owned(),
                version_args: vec!["leptos".to_owned(), "--version".to_owned()],
                install_hint: format!(
                    "cargo install cargo-leptos --locked --version {CARGO_LEPTOS_VERSION}"
                ),
            }],
        }
    }

    /// The adapter has no settings yet, so any are reported rather than
    /// silently ignored.
    fn validate(&self, component: &Component) -> Vec<Diagnostic> {
        component
            .config
            .iter()
            .filter(|(_, settings)| !settings.is_empty())
            .map(|(integration, _)| {
                Diagnostic::error(
                    Code::UnsupportedSetting,
                    format!("config.{integration}"),
                    format!(
                        "adapter `{ADAPTER_ID}` has no `{integration}` settings yet; remove this table"
                    ),
                )
            })
            .collect()
    }

    fn plan(&self, operation: Operation, component: &Component) -> Result<Vec<Step>, Diagnostic> {
        let Some(cwd) = component.path.clone() else {
            return Err(Diagnostic::error(
                Code::UnsupportedOperation,
                "",
                format!(
                    "component `{}` needs a `path` to build; external components cannot be built",
                    component.id
                ),
            ));
        };
        let downloads = Effect::Network("crates.io".to_owned());
        let target = Effect::WritesDir("target".to_owned());

        let step = match operation {
            Operation::Build => Step {
                id: format!("{}.build", component.id),
                component: component.id.clone(),
                description: "build the server and the WebAssembly client with cargo-leptos"
                    .to_owned(),
                program: "cargo".to_owned(),
                args: vec!["leptos".into(), "build".into(), "--release".into()],
                cwd,
                env: Vec::new(),
                effects: vec![target, downloads],
                long_running: false,
            },
            Operation::DevStart => Step {
                id: format!("{}.dev", component.id),
                component: component.id.clone(),
                description: "serve the app and rebuild on changes with cargo-leptos".to_owned(),
                program: "cargo".to_owned(),
                args: vec!["leptos".into(), "watch".into()],
                cwd,
                env: vec!["LEPTOS_SITE_ADDR".to_owned()],
                effects: vec![
                    target,
                    downloads,
                    Effect::ListensOn(DEFAULT_SITE_ADDR.to_owned()),
                ],
                long_running: true,
            },
        };
        Ok(vec![step])
    }
}
