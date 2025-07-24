use crate::store::viewport::use_viewport;
use dioxus::prelude::*;

#[component]
pub fn CursorGlow() -> Element {
    let viewport = use_viewport();
    let mouse_pos = viewport.mouse_position.read();

    let (x, y) = (mouse_pos.x, mouse_pos.y);

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
