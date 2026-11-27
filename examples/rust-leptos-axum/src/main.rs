#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use framestack_reference_leptos_axum::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).expect("Leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        // A plain Axum route next to the Leptos pages, for health checks.
        .route("/healthz", get(|| async { "ok" }))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind the site address");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("serve");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // The browser build has no main; see `hydrate` in lib.rs.
}
