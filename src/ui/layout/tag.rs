use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    text: String,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    rsx! {
        span {
            class: "px-4 py-1.5 border border-white/30 text-xs opacity-60 uppercase tracking-wider transition-all duration-300 ease-in-out hover:opacity-100 hover:border-white text-white",
            "{props.text}"
        }
    }
}