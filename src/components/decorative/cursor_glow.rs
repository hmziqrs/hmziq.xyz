use dioxus::{logger::tracing, prelude::*};
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
        // "transform: translate(-50%, -50%); left: {}px; top: {}px; background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%); filter: blur(64px);",
        "transform: translate(-50%, -50%); left: {}px; top: {}px;",
        x, y
    );

    // tracing::info!("x,y: {},{}", x, y);

    rsx! {
        div {
            class: "fixed pointer-events-none z-50 w-96 h-96 rounded-full bg-[radial-gradient(circle,_rgba(255,255,255,0.1)_0%,_transparent_70%)] transition-transform duration-200 blur-lg ease-out",
            // class: "fixed pointer-events-none z-50 w-24 h-24 rounded-full bg-white/10 transition-transform duration-200 blur-xl ease-out",

            // class: "fixed pointer-events-none z-50 w-96 h-96 rounded-full bg-gradient-radial from-white via-white to-transparent blur-sm transition-transform duration-200 ease",
            style: "{glow_style}",
        }
    }
}
