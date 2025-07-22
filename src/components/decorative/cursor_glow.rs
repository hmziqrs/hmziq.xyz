use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CursorGlowProps {
    position: (f64, f64),
}

#[component]
pub fn CursorGlow(props: CursorGlowProps) -> Element {
    let (x, y) = props.position;
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
