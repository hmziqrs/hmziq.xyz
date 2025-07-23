use dioxus::prelude::*;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

impl Default for MousePosition {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScrollOffset {
    pub y: f64,
}

impl Default for ScrollOffset {
    fn default() -> Self {
        Self { y: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowDimensions {
    pub width: f64,
    pub height: f64,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 800.0,
        }
    }
}

pub struct ViewportState {
    pub mouse_position: GlobalSignal<MousePosition>,
    pub scroll_offset: GlobalSignal<ScrollOffset>,
    pub window_dimensions: GlobalSignal<WindowDimensions>,
}

static VIEWPORT_STATE: OnceLock<ViewportState> = OnceLock::new();

pub fn use_viewport() -> &'static ViewportState {
    VIEWPORT_STATE.get_or_init(|| ViewportState::new())
}
