use gpui::{Div, InteractiveElement, IntoElement, ParentElement, ScrollHandle, Stateful, StatefulInteractiveElement, Styled, StyledText, div, rgb};

use crate::ui::config::colors::{TERMINAL_BG, TERMINAL_BORDER};

#[derive(Clone)]
pub enum TerminalLine {
    Normal(String),
    Error(String),
}

pub fn terminal_view() -> TerminalView {
    TerminalView {
        buffer: vec![],
        scroll: ScrollHandle::new(),
    }
}

#[derive(Clone)]
pub struct TerminalView {
    buffer: Vec<TerminalLine>,
    scroll: ScrollHandle,
}

impl TerminalView {
    pub fn new(buffer: Vec<TerminalLine>) -> Self {
        Self {
            buffer,
            scroll: ScrollHandle::new(),
        }
    }

    pub fn append(&mut self, line: TerminalLine) {
        self.buffer.push(line);
        self.scroll.scroll_to_bottom();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn buffer_to_text(self) -> Vec<Div> {
        self.buffer.iter().map(|line| {
            match line {
                TerminalLine::Normal(string) => div().child(string.clone()),
                TerminalLine::Error(string) => div().child(string.clone()).text_color(rgb(0xFF0000)),
            }
        }).collect()
    }
}

impl IntoElement for TerminalView {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        div()
            .id("terminal_view")
            .bg(rgb(TERMINAL_BG))
            .border_1()
            .border_color(rgb(TERMINAL_BORDER))
            .rounded_sm()
            .size_full()
            .px_2()
            .py_1()
            .font_family("JetBrains Mono, Fira Code, monospace")
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .child(
                div()
                    .children(self.buffer_to_text())
            )
    }
}
