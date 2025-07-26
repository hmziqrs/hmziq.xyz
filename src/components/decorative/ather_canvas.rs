use dioxus::prelude::*;

#[component]
pub fn AtherCanvas() -> Element {
    rsx! {
        canvas {
            id: "ather-canvas",
            class: "absolute inset-0 w-full h-full",
            style: "z-index: 0;"
        }
    }
}