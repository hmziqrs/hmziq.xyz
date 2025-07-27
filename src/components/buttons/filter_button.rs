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
    let base_classes = "px-[30px] py-3 border transition-all duration-300 ease-in-out relative overflow-hidden tracking-[0.1rem] uppercase text-xs backdrop-blur-sm bg-black/10";

    let state_classes = if props.active {
        "border-white text-black shadow-[0_0_20px_rgba(255,255,255,0.3)]"
    } else {
        "border-white/30 text-white hover:text-black hover:border-white hover:shadow-[0_0_20px_rgba(255,255,255,0.3)]"
    };

    let after_classes = if props.active {
        "after:w-[160px] after:h-[160px]"
    } else {
        "after:w-0 after:h-0 hover:after:w-[160px] hover:after:h-[160px]"
    };

    rsx! {
        button {
            class: "{base_classes} {state_classes} {after_classes} after:content-[''] after:absolute after:top-1/2 after:left-1/2 after:bg-white after:transition-all after:duration-300 after:ease-in-out after:-translate-x-1/2 after:-translate-y-1/2 after:rounded-full cursor-none btn",
            onclick: move |_| props.on_click.call(props.filter.clone()),

            span {
                class: "relative z-10",
                "{props.text}"
            }
        }
    }
}
