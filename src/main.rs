use dioxus::prelude::*;

pub mod components;
pub mod containers;
pub mod hooks;
pub mod router;
pub mod screens;
// pub mod services;
mod config;
mod env;
pub mod store;
pub mod types;
pub mod ui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const APP_JS: Asset = asset!("/assets/app.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: APP_JS }
        Router::<crate::router::Route> {}
    }
}
