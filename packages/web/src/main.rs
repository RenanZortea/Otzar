#![allow(non_snake_case)]

use dioxus::prelude::*;
use views::Home;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// 1. Server Entrypoint (Runs on Shuttle)
#[cfg(feature = "server")]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // FIX 1: Use the axum re-exported by shuttle_axum to avoid version conflicts
    use shuttle_axum::axum::Router;

    // FIX 2: Use ServeConfig::new() instead of builder pattern for Dioxus 0.7+
    // We unwrap() because new() returns a Result
    let config = ServeConfig::new().unwrap();

    let router = Router::new()
        .serve_dioxus_application(config, App);

    Ok(router.into())
}

// 2. Client Entrypoint (Runs in Browser)
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

// Shared App Component
#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
