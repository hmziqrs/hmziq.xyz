use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OrbitProps {
    size: String,
    duration: String,
    reverse: bool,
    #[props(default = "".to_string())]
    class: String,
}

#[component]
pub fn Orbit(props: OrbitProps) -> Element {
    let rotation_direction = if props.reverse { "reverse" } else { "normal" };
    let orbit_style = format!(
        "width: {}; height: {}; animation: spin {} linear infinite {}; animation-direction: {};",
        props.size, props.size, props.duration, rotation_direction, rotation_direction
    );

    rsx! {
        div {
            class: "absolute border border-white/10 rounded-full pointer-events-none {props.class}",
            style: "{orbit_style}",
            
            // Planet/satellite
            div {
                class: "absolute w-5 h-5 bg-white rounded-full -top-2.5 left-1/2 -translate-x-1/2 shadow-[0_0_20px_rgba(255,255,255,0.8)]"
            }
        }
    }
}