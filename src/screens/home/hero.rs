use crate::{
    components::{buttons::ExploreButton, decorative::Orbit},
    ui::GlitchTitle,
};
use dioxus::prelude::*;

#[component]
pub fn HeroSection() -> Element {
    rsx! {
        section {
            class: "h-screen relative flex items-center justify-center overflow-hidden",



            // Orbiting elements
            Orbit {
                size: "300px",
                duration: "60s",
                reverse: false,
                class: "top-[10%] -right-[150px]"
            }
            Orbit {
                size: "500px",
                duration: "90s",
                reverse: true,
                class: "-bottom-[250px] -left-[250px]"
            }

            // Hero content
            div {
                class: "text-center z-10 relative",

                GlitchTitle { text: "HMZIQ LABS" }

                p {
                    class: "text-base sm:text-xl lg:text-2xl font-light tracking-[0.5rem] opacity-80 animate-fadeInUp animation-delay-300 uppercase",
                    "Experiments"
                }

                div {
                    class: "mt-16 animate-fadeInUp animation-delay-600",
                    ExploreButton {
                        text: "Explore",
                        href: "#projects"
                    }
                }
            }

        }
    }
}
