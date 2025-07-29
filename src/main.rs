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

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const APP_JS: Asset = asset!("/assets/app.js");

fn main() {
    router::create_sitemap();

    // dioxus::launch(App);
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            dioxus::server::ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus_isrg::IncrementalRendererConfig::new()
                        .static_dir(router::static_dir())
                        .clear_cache(false)
                        // .pre_render(true)

                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: APP_JS }
        Router::<crate::router::Route> {}
    }
}
