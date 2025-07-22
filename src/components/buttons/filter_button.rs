use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterButtonProps {
    text: String,
    filter: String,
    active: bool,
    on_click: EventHandler<String>,
}

#[component]
pub fn FilterButton(props: FilterButtonProps) -> Element {
    let base_classes = "px-8 py-3 border transition-all duration-300 ease-in-out relative overflow-hidden tracking-wider uppercase text-xs backdrop-blur-sm cursor-pointer";
    
    let state_classes = if props.active {
        "border-white bg-white text-black shadow-[0_0_20px_rgba(255,255,255,0.3)]"
    } else {
        "border-white/30 bg-black/50 text-white hover:text-black hover:border-white hover:shadow-[0_0_20px_rgba(255,255,255,0.3)] hover:bg-white"
    };

    rsx! {
        button {
            class: "{base_classes} {state_classes}",
            onclick: move |_| props.on_click.call(props.filter.clone()),
            "{props.text}"
        }
    }
}