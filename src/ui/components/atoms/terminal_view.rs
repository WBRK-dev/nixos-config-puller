use gpui::{Div, IntoElement, ParentElement, Styled, div, rgb};

use crate::ui::config::colors::{TERMINAL_BG, TERMINAL_BORDER};

pub fn terminal_view() -> TerminalView {
    TerminalView {
        buffer: String::new(),
    }
}

#[derive(Clone)]
pub struct TerminalView {
    buffer: String,
}

impl TerminalView {
    pub fn new(buffer: String) -> Self {
        Self {
            buffer
        }
    }

    pub fn append(&mut self, chunk: &str) {
        self.buffer.push_str("\n");
        self.buffer.push_str(chunk);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl IntoElement for TerminalView {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .bg(rgb(TERMINAL_BG))
            .border_1()
            .border_color(rgb(TERMINAL_BORDER))
            .rounded_sm()
            .size_full()
            .font_family("JetBrains Mono, Fira Code, monospace")
            .overflow_hidden()
            .child(self.buffer)
    }
}
