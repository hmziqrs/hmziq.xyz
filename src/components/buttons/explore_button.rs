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
            class: "inline-block cursor-none px-18 py-6 border border-white text-white hover:text-black transition-all duration-300 ease relative overflow-hidden tracking-wider uppercase text-sm hover:shadow-[0_0_30px_rgba(255,255,255,0.5)] group",

            // Expanding circle background
            div {
                class: "absolute top-1/2 left-1/2 w-0 h-0 bg-white rounded-full hover:rounded-full transition-all duration-300 ease transform -translate-x-1/2 -translate-y-1/2 group-hover:w-[250px] group-hover:h-[250px]"
            }

            // Text content
            span {
                class: "relative z-10 tracking-[0.2rem]",
                "{props.text}"
            }
        }
    }
}
