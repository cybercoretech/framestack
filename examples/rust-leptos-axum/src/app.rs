use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/framestack-reference-leptos-axum.css"/>
        <Title text="FrameStack reference: Leptos + Axum"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// The server renders the page with the count at 0. The button only counts
/// once the WebAssembly client has hydrated the page.
#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    // Effects run only in the browser, so the server always sends "Not
    // hydrated yet" and only a working client can change it. CI checks for
    // the hydrated text (scripts/smoke-leptos-axum.sh).
    let hydrated = RwSignal::new(false);
    Effect::new(move |_| hydrated.set(true));

    view! {
        <h1>"FrameStack reference: Leptos + Axum"</h1>
        <p>"Rendered on the server by Axum, then hydrated in the browser."</p>
        <button on:click=on_click>{move || format!("Clicked {} times", count.get())}</button>
        <p id="hydration">
            {move || if hydrated.get() { "Hydrated in the browser" } else { "Not hydrated yet" }}
        </p>
    }
}
