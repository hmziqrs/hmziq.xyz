use super::state::{ViewportState, MousePosition, ScrollOffset, WindowDimensions};
use dioxus::prelude::*;

impl ViewportState {
    pub fn new() -> Self {
        ViewportState {
            mouse_position: GlobalSignal::new(|| MousePosition::default()),
            scroll_offset: GlobalSignal::new(|| ScrollOffset::default()),
            window_dimensions: GlobalSignal::new(|| WindowDimensions::default()),
        }
    }

    pub fn update_mouse_position(&self, x: f64, y: f64) {
        *self.mouse_position.write() = MousePosition { x, y };
    }

    pub fn update_scroll_offset(&self, y: f64) {
        *self.scroll_offset.write() = ScrollOffset { y };
    }

    pub fn update_window_dimensions(&self, width: f64, height: f64) {
        *self.window_dimensions.write() = WindowDimensions { width, height };
    }
}