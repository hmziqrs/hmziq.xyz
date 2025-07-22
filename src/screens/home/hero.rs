use crate::{
    components::{
        buttons::ExploreButton,
        decorative::{Nebula, Orbit},
    },
    ui::GlitchTitle,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroSectionProps {
    scroll_y: f64,
}

#[component]
pub fn HeroSection(props: HeroSectionProps) -> Element {
    rsx! {
        section {
            class: "h-screen relative flex items-center justify-center overflow-hidden",

            // Parallax nebulas
            Nebula {
                width: "600px",
                height: "600px",
                animation_duration: "20s",
                style: format!("transform: translate(-50%, {}%)", -50.0 + props.scroll_y * 0.1)
            }
            Nebula {
                width: "800px",
                height: "800px",
                animation_duration: "30s",
                style: format!("transform: translate(-50%, {}%)", -50.0 + props.scroll_y * 0.15)
            }

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

                GlitchTitle { text: "DIGITAL LABORATORY" }

                p {
                    class: "text-base sm:text-xl lg:text-2xl font-light tracking-[0.5rem] opacity-80 animate-fadeInUp animation-delay-300 uppercase",
                    "Experiments in Code & Design"
                }

                div {
                    class: "mt-16 animate-fadeInUp animation-delay-600",
                    ExploreButton {
                        text: "INITIATE EXPLORATION",
                        href: "#projects"
                    }
                }
            }

            // Coordinates display
            div {
                class: "absolute bottom-8 left-8 font-mono text-xs opacity-50 tracking-[0.1rem]",
                span { "COORD: " }
                span { id: "coordText", "00.000° N, 00.000° W" }
            }
        }
    }
}
