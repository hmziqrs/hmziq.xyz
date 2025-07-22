use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GlitchTitleProps {
    text: String,
}

#[component]
pub fn GlitchTitle(props: GlitchTitleProps) -> Element {
    rsx! {
        h1 {
            class: "text-5xl sm:text-7xl lg:text-8xl font-thin tracking-[1rem] mb-4 relative text-white glitch-container",
            style: "text-shadow: 0 0 20px rgba(255,255,255,0.5);",
            
            // Main text
            span {
                class: "relative z-30",
                "{props.text}"
            }
            
            // Glitch layer 1 (red)
            span {
                class: "absolute top-0 left-0 w-full h-full z-20 text-red-500 opacity-80",
                style: "
                    animation: glitch-1 2s infinite linear alternate-reverse;
                    clip-path: polygon(0 0, 100% 0, 100% 5%, 0 5%);
                ",
                "{props.text}"
            }
            
            // Glitch layer 2 (cyan)
            span {
                class: "absolute top-0 left-0 w-full h-full z-10 text-cyan-400 opacity-80",
                style: "
                    animation: glitch-2 3s infinite linear alternate-reverse;
                    clip-path: polygon(0 10%, 100% 10%, 100% 44%, 0 44%);
                ",
                "{props.text}"
            }
            
            // Glitch layer 3 (yellow)
            span {
                class: "absolute top-0 left-0 w-full h-full z-10 text-yellow-300 opacity-60",
                style: "
                    animation: glitch-3 5s infinite linear alternate-reverse;
                    clip-path: polygon(0 70%, 100% 70%, 100% 85%, 0 85%);
                ",
                "{props.text}"
            }
        }
        
        // Inline CSS for glitch animations
        style {
            r#"
            @keyframes glitch-1 {{
                0% {{
                    clip-path: polygon(0 0%, 100% 0%, 100% 5%, 0 5%);
                    transform: translate(-2px, -1px);
                }}
                10% {{
                    clip-path: polygon(0 15%, 100% 15%, 100% 30%, 0 30%);
                    transform: translate(1px, 1px);
                }}
                20% {{
                    clip-path: polygon(0 10%, 100% 10%, 100% 20%, 0 20%);
                    transform: translate(-1px, 2px);
                }}
                30% {{
                    clip-path: polygon(0 1%, 100% 1%, 100% 2%, 0 2%);
                    transform: translate(1px, -1px);
                }}
                40% {{
                    clip-path: polygon(0 35%, 100% 35%, 100% 50%, 0 50%);
                    transform: translate(-2px, 1px);
                }}
                50% {{
                    clip-path: polygon(0 45%, 100% 45%, 100% 46%, 0 46%);
                    transform: translate(2px, -2px);
                }}
                60% {{
                    clip-path: polygon(0 50%, 100% 50%, 100% 70%, 0 70%);
                    transform: translate(-1px, 1px);
                }}
                70% {{
                    clip-path: polygon(0 70%, 100% 70%, 100% 75%, 0 75%);
                    transform: translate(1px, -1px);
                }}
                80% {{
                    clip-path: polygon(0 2%, 100% 2%, 100% 5%, 0 5%);
                    transform: translate(-2px, 2px);
                }}
                90% {{
                    clip-path: polygon(0 75%, 100% 75%, 100% 100%, 0 100%);
                    transform: translate(2px, -1px);
                }}
                100% {{
                    clip-path: polygon(0 0%, 100% 0%, 100% 5%, 0 5%);
                    transform: translate(-2px, -1px);
                }}
            }}
            
            @keyframes glitch-2 {{
                0% {{
                    clip-path: polygon(0 25%, 100% 25%, 100% 30%, 0 30%);
                    transform: translate(2px, 1px);
                }}
                15% {{
                    clip-path: polygon(0 3%, 100% 3%, 100% 3%, 0 3%);
                    transform: translate(-1px, -2px);
                }}
                25% {{
                    clip-path: polygon(0 5%, 100% 5%, 100% 20%, 0 20%);
                    transform: translate(1px, 2px);
                }}
                35% {{
                    clip-path: polygon(0 20%, 100% 20%, 100% 25%, 0 25%);
                    transform: translate(-2px, 1px);
                }}
                45% {{
                    clip-path: polygon(0 40%, 100% 40%, 100% 45%, 0 45%);
                    transform: translate(2px, -1px);
                }}
                55% {{
                    clip-path: polygon(0 45%, 100% 45%, 100% 60%, 0 60%);
                    transform: translate(-1px, 2px);
                }}
                65% {{
                    clip-path: polygon(0 60%, 100% 60%, 100% 70%, 0 70%);
                    transform: translate(1px, -2px);
                }}
                75% {{
                    clip-path: polygon(0 70%, 100% 70%, 100% 80%, 0 80%);
                    transform: translate(-2px, 1px);
                }}
                85% {{
                    clip-path: polygon(0 80%, 100% 80%, 100% 90%, 0 90%);
                    transform: translate(2px, -1px);
                }}
                100% {{
                    clip-path: polygon(0 25%, 100% 25%, 100% 30%, 0 30%);
                    transform: translate(2px, 1px);
                }}
            }}
            
            @keyframes glitch-3 {{
                0% {{
                    clip-path: polygon(0 75%, 100% 75%, 100% 80%, 0 80%);
                    transform: translate(-1px, 1px);
                }}
                20% {{
                    clip-path: polygon(0 85%, 100% 85%, 100% 95%, 0 95%);
                    transform: translate(1px, -1px);
                }}
                40% {{
                    clip-path: polygon(0 90%, 100% 90%, 100% 95%, 0 95%);
                    transform: translate(-1px, 2px);
                }}
                60% {{
                    clip-path: polygon(0 70%, 100% 70%, 100% 85%, 0 85%);
                    transform: translate(2px, -2px);
                }}
                80% {{
                    clip-path: polygon(0 80%, 100% 80%, 100% 100%, 0 100%);
                    transform: translate(-2px, 1px);
                }}
                100% {{
                    clip-path: polygon(0 75%, 100% 75%, 100% 80%, 0 80%);
                    transform: translate(-1px, 1px);
                }}
            }}
            "#
        }
    }
}