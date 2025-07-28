use dioxus::prelude::*;

#[component]
pub fn CursorGlow() -> Element {
    rsx! {
        div {
            id: "cursor",
            class: "fixed pointer-events-none z-[9999] rounded-full w-[20px] h-[20px] border border-white transition-all duration-100 ease",
            style: "mix-blend-mode: difference;",
            // style: "mix-blend-mode: difference; box-shadow: 0 0 20px rgba(255,255,255,1.0);",
        }
        div {
            id: "cursor-trail",
            class: "fixed pointer-events-none z-[9999] rounded-full w-[6px] h-[6px] shadow-[0_0_10px_rgba(255,255,255,1.0)] bg-white transition-all duration-200 ease",
            style: "mix-blend-mode: difference;",
        }
        // div {
        //     class: "fixed pointer-events-none z-50 w-96 h-96 rounded-full bg-[radial-gradient(circle,_rgba(255,255,255,0.1)_0%,_transparent_70%)] transition-transform duration-200 blur-lg ease-out",
        //     // class: "fixed pointer-events-none z-50 w-24 h-24 rounded-full bg-white/10 transition-transform duration-200 blur-xl ease-out",

        //     // class: "fixed pointer-events-none z-50 w-96 h-96 rounded-full bg-gradient-radial from-white via-white to-transparent blur-sm transition-transform duration-200 ease",
        //     style: "{glow_style}",
        // }
    }
}
