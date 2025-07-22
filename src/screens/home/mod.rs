use crate::components::decorative::CursorGlow;
use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};

mod contact;
mod hero;
mod projects;

pub use contact::ContactSection;
pub use hero::HeroSection;
pub use projects::ProjectsSection;

#[component]
pub fn HomeScreen() -> Element {
    // Scroll position for parallax
    let mut scroll_y = use_signal(|| 0.0);

    // Mouse position for cursor effects
    let mut mouse_pos = use_signal(|| (0.0, 0.0));

    // Active filter state
    let active_filter = use_signal(|| "all".to_string());

    // Scroll event handler
    use_effect(move || {
        let window = web_sys::window().expect("should have window");
        let window_clone = window.clone();

        let scroll_handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let scroll_y_val = window_clone.page_y_offset().unwrap_or(0.0);
            scroll_y.set(scroll_y_val);
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref())
            .expect("should add scroll listener");

        scroll_handler.forget(); // Keep closure alive
    });

    // Mouse event handler
    use_effect(move || {
        let window = web_sys::window().expect("should have window");
        let document = window.document().expect("should have document");
        let window_clone = window.clone();
        let document_clone = document.clone();

        let mouse_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            mouse_pos.set((event.client_x() as f64, event.client_y() as f64));

            // Update coordinates display
            if let Some(coord_elem) = document_clone.get_element_by_id("coordText") {
                let window_height = window_clone
                    .inner_height()
                    .unwrap()
                    .as_f64()
                    .unwrap_or(800.0);
                let window_width = window_clone
                    .inner_width()
                    .unwrap()
                    .as_f64()
                    .unwrap_or(1200.0);

                let lat = event.client_y() as f64 / window_height * 180.0 - 90.0;
                let lng = event.client_x() as f64 / window_width * 360.0 - 180.0;
                let coord_text = format!(
                    "{:.3}° {}, {:.3}° {}",
                    lat.abs(),
                    if lat > 0.0 { "N" } else { "S" },
                    lng.abs(),
                    if lng > 0.0 { "E" } else { "W" }
                );
                coord_elem.set_inner_html(&coord_text);
            }
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("mousemove", mouse_handler.as_ref().unchecked_ref())
            .expect("should add mouse listener");

        mouse_handler.forget(); // Keep closure alive
    });

    rsx! {
        div {
            class: "bg-black text-white overflow-x-hidden leading-relaxed cursor-crosshair min-h-screen",

            // Canvas placeholder - will be implemented later
            div { id: "space-canvas", class: "fixed inset-0 -z-10" }

            // Cursor glow effect
            CursorGlow { position: mouse_pos() }

            // Sections
            HeroSection { scroll_y: scroll_y() }
            ProjectsSection { active_filter }
            ContactSection {}

            // Floating asteroids
            Asteroids {}
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
