use dioxus::prelude::*;

#[component]
pub fn Coordinates() -> Element {
    rsx! {
        div {
            id: "coordinates",
            class: "absolute top-8 left-8 z-50 px-4 py-2 backdrop-blur-sm bg-black/20 border border-white/10 rounded-lg duration-300 ease transition-transform",
            div {
                class: "font-mono text-xs opacity-70 tracking-[0.1rem] text-white",
                span { "COORD: " }
                span { class: "coord-text", "0.000° N, 0.000° E" }
            }
        }
    }
}
