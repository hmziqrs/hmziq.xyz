use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

#[component]
pub fn CursorGlow() -> Element {
    let mut mouse_pos = use_signal(|| (0.0, 0.0));

    let (x, y) = mouse_pos();

    use_effect(move || {
        let window = web_sys::window().expect("should have window");
        let document = window.document().expect("should have document");

        let mouse_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            mouse_pos.set((event.client_x() as f64, event.client_y() as f64));
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("mousemove", mouse_handler.as_ref().unchecked_ref())
            .expect("should add mouse listener");

        mouse_handler.forget(); // Keep closure alive
    });

    let glow_style = format!(
        "transform: translate(-50%, -50%); left: {}px; top: {}px;",
        x, y
    );

    rsx! {
        div {
            class: "fixed pointer-events-none z-50 w-96 h-96 rounded-full bg-gradient-radial from-white/10 via-white/5 to-transparent blur-3xl transition-all duration-100 ease-out",
            style: "{glow_style}",
        }
    }
}
