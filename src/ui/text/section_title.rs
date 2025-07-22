use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionTitleProps {
    text: String,
}

#[component]
pub fn SectionTitle(props: SectionTitleProps) -> Element {
    rsx! {
        div {
            class: "relative inline-block",
            
            // Left decorative line
            div {
                class: "absolute h-px w-24 bg-white top-1/2 transform -translate-y-1/2",
                style: "right: calc(100% + 30px);"
            }
            
            // Title text
            h2 {
                class: "text-4xl sm:text-5xl lg:text-6xl font-thin tracking-[0.5rem] mb-8 text-white",
                "{props.text}"
            }
            
            // Right decorative line
            div {
                class: "absolute h-px w-24 bg-white top-1/2 transform -translate-y-1/2",
                style: "left: calc(100% + 30px);"
            }
        }
    }
}