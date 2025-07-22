use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContactCardProps {
    icon: String,
    label: String,
    href: String,
}

#[component]
pub fn ContactCard(props: ContactCardProps) -> Element {
    rsx! {
        a {
            href: "{props.href}",
            class: "block p-10 border border-white/30 transition-all duration-500 ease-in-out text-white relative overflow-hidden bg-black/50 backdrop-blur-sm hover:translate-y-[-10px] hover:shadow-[0_20px_40px_rgba(255,255,255,0.2)] hover:border-white hover:text-black text-center group",
            
            // Expanding circular background on hover
            div {
                class: "absolute top-1/2 left-1/2 w-0 h-0 bg-white rounded-full transition-all duration-500 ease-in-out transform -translate-x-1/2 -translate-y-1/2 group-hover:w-[200px] group-hover:h-[200px]"
            }
            
            // Icon
            div {
                class: "text-5xl mb-6 transition-all duration-300 ease-in-out relative z-10 group-hover:scale-110",
                "{props.icon}"
            }
            
            // Label
            div {
                class: "text-lg font-light tracking-wider uppercase relative z-10",
                "{props.label}"
            }
        }
    }
}