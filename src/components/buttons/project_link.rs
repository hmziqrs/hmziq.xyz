use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectLinkProps {
    text: String,
    href: String,
}

#[component]
pub fn ProjectLink(props: ProjectLinkProps) -> Element {
    rsx! {
        a {
            href: "{props.href}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "text-white no-underline px-6 py-2.5 border border-white/30 transition-all duration-300 ease-in-out text-xs uppercase tracking-[0.1rem] relative overflow-hidden hover:text-black hover:border-white group/link btn cursor-none",

            // Bottom-to-top fill background
            div {
                class: "absolute bottom-0 left-0 w-full h-0 bg-white transition-all duration-300 ease-in-out group-hover/link:h-full"
            }

            // Text content
            span {
                class: "relative z-10",
                "{props.text}"
            }
        }
    }
}
