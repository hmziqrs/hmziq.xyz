use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ExploreButtonProps {
    text: String,
    href: String,
}

#[component]
pub fn ExploreButton(props: ExploreButtonProps) -> Element {
    rsx! {
        a {
            href: "{props.href}",
            class: "inline-block px-14 py-5 border border-white text-white hover:text-black transition-all duration-300 ease-in-out relative overflow-hidden tracking-wider uppercase text-sm hover:shadow-[0_0_30px_rgba(255,255,255,0.5)] group",
            
            // Expanding circle background
            div {
                class: "absolute top-1/2 left-1/2 w-0 h-0 bg-white rounded-full transition-all duration-300 ease-in-out transform -translate-x-1/2 -translate-y-1/2 group-hover:w-[300px] group-hover:h-[300px]"
            }
            
            // Text content
            span {
                class: "relative z-10",
                "{props.text}"
            }
        }
    }
}