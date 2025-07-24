use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};

mod contact;
mod hero;
mod projects;

pub use contact::ContactSection;
pub use hero::HeroSection;
pub use projects::ProjectsSection;

use crate::components::{decorative::Coordinates, CursorGlow};
use crate::store::viewport::use_viewport;

#[component]
pub fn HomeScreen() -> Element {
    let viewport = use_viewport();

    // Active filter state
    let active_filter = use_signal(|| "all".to_string());

    // Scroll event handler
    use_effect(move || {
        let window = web_sys::window().expect("should have window");
        let window_clone = window.clone();

        let scroll_handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let scroll_y_val = window_clone.page_y_offset().unwrap_or(0.0);
            viewport.update_scroll_offset(scroll_y_val);
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref())
            .expect("should add scroll listener");

        scroll_handler.forget(); // Keep closure alive
    });

    // Mouse event handler
    use_effect(move || {
        let document = web_sys::window()
            .expect("should have window")
            .document()
            .expect("should have document");

        let mouse_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            viewport.update_mouse_position(event.client_x() as f64, event.client_y() as f64);
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("mousemove", mouse_handler.as_ref().unchecked_ref())
            .expect("should add mouse listener");

        mouse_handler.forget(); // Keep closure alive
    });

    // Window resize event handler and initial dimensions
    use_effect(move || {
        let window = web_sys::window().expect("should have window");
        let window_clone = window.clone();

        // Set initial dimensions
        let initial_width = window.inner_width().unwrap().as_f64().unwrap_or(1200.0);
        let initial_height = window.inner_height().unwrap().as_f64().unwrap_or(800.0);
        viewport.update_window_dimensions(initial_width, initial_height);

        let resize_handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let width = window_clone
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap_or(1200.0);
            let height = window_clone
                .inner_height()
                .unwrap()
                .as_f64()
                .unwrap_or(800.0);
            viewport.update_window_dimensions(width, height);
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref())
            .expect("should add resize listener");

        resize_handler.forget(); // Keep closure alive
    });

    rsx! {
        div {
            class: "bg-black text-white overflow-x-hidden leading-relaxed min-h-screen cursor-none",

            Asteroids {}
            // Canvas placeholder - will be implemented later
            div { id: "space-canvas", class: "fixed inset-0 -z-10" }
            CursorGlow { }
            Coordinates { }
            // Sections
            HeroSection { scroll_y: viewport.scroll_offset.read().y }
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
