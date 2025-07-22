use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NebulaProps {
    #[props(default = "600px".to_string())]
    width: String,
    #[props(default = "600px".to_string())]
    height: String,
    #[props(default = "20s".to_string())]
    animation_duration: String,
    #[props(default = "".to_string())]
    style: String,
}

#[component]
pub fn Nebula(props: NebulaProps) -> Element {
    let inline_style = format!(
        "width: {}; height: {}; animation-duration: {}; {}",
        props.width, props.height, props.animation_duration, props.style
    );

    rsx! {
        div {
            class: "absolute blur-[100px] bg-gradient-radial from-white/5 to-transparent pointer-events-none",
            style: "{inline_style}; animation: float {props.animation_duration} ease-in-out infinite;",
        }
        
        // Inline CSS for float animation if not already defined
        style {
            r#"
            @keyframes float {
                0%, 100% {
                    transform: translate(-50%, -50%) scale(1);
                }
                50% {
                    transform: translate(-30%, -70%) scale(1.2);
                }
            }
            "#
        }
    }
}