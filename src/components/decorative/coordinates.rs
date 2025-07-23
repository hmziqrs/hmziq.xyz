use crate::store::viewport::use_viewport;
use dioxus::prelude::*;

#[component]
pub fn Coordinates() -> Element {
    let viewport = use_viewport();
    let scroll_offset = viewport.scroll_offset.read();

    let sticky_style = format!("transform: translate(0, {}px);", scroll_offset.y);

    // Calculate lat/lng from mouse position - read signals inside memo
    let coord_text = use_memo(move || {
        let mouse_pos = viewport.mouse_position.read();
        let window_dims = viewport.window_dimensions.read();

        let lat = mouse_pos.y / window_dims.height * 180.0 - 90.0;
        let lng = mouse_pos.x / window_dims.width * 360.0 - 180.0;

        format!(
            "{:.3}° {}, {:.3}° {}",
            lat.abs(),
            if lat > 0.0 { "N" } else { "S" },
            lng.abs(),
            if lng > 0.0 { "E" } else { "W" }
        )
    });

    rsx! {
        div {
            class: "absolute top-8 left-8 z-50 px-4 py-2 backdrop-blur-sm bg-black/20 border border-white/10 rounded-lg duration-300 ease transition-transform",
            style: "{sticky_style}",
            div {
                class: "font-mono text-xs opacity-70 tracking-[0.1rem] text-white",
                span { "COORD: " }
                span { "{coord_text}" }
            }
        }
    }
}
