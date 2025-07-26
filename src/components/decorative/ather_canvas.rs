use dioxus::prelude::*;

#[component]
pub fn AtherCanvas() -> Element {
    rsx! {
        div {
            id: "ather-wrapper",
            class: "absolute inset-0 w-full h-full opacity-0 transition-opacity duration-1000 ease-in-out",
            style: "z-index: 0;",
            canvas {
                id: "ather-canvas",
                class: "w-full h-full"
            }
        }
    }
}
