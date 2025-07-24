use dioxus::prelude::*;

mod contact;
mod hero;
mod projects;

pub use contact::ContactSection;
pub use hero::HeroSection;
pub use projects::ProjectsSection;

use crate::components::{decorative::Coordinates, CursorGlow};

#[component]
pub fn HomeScreen() -> Element {
    // Active filter state
    let active_filter = use_signal(|| "all".to_string());

    rsx! {
        div {
            class: "bg-black text-white overflow-x-hidden leading-relaxed min-h-screen cursor-none",

            Asteroids {}
            // Canvas placeholder - will be implemented later
            div { id: "space-canvas", class: "fixed inset-0 -z-10" }
            CursorGlow { }
            Coordinates { }
            // Sections
            HeroSection { }
            ProjectsSection { active_filter }
            ContactSection {}

            // Floating asteroids
        }
    }
}

#[component]
fn Asteroids() -> Element {
    rsx! {
        for i in 0..10 {
            div {
                class: "fixed w-[3px] h-[3px] bg-white rounded-full opacity-60 animate-asteroid pointer-events-none",
                style: format!(
                    "top: {}%; animation-delay: {}s; animation-duration: {}s",
                    (i * 10) % 100,
                    i as f32 * 2.0,
                    15.0 + (i as f32 * 0.5)
                )
            }
        }
    }
}
