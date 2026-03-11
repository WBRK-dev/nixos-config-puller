use gpui::{App, Div, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement, Styled, Window, div, rgb};

use crate::ui::config::colors::{PRIMARY_BG, PRIMARY_BG_HOVER, PRIMARY_TEXT, SECONDARY_BG, SECONDARY_BG_HOVER, SECONDARY_TEXT};

pub enum ButtonTheme {
    Primary,
    Secondary,
}

type OnClick = Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>;

pub fn button() -> Button {
    Button {
        text: "Button",
        theme: ButtonTheme::Primary,
        on_click: None,
    }
}

pub struct Button {
    text: &'static str,
    theme: ButtonTheme,
    on_click: Option<OnClick>,
}

impl Button {
    pub fn text(mut self, text: &'static str) -> Self {
        self.text = text;
        self
    }

    pub fn theme(mut self, theme: ButtonTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl IntoElement for Button {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let elem = div()
            .cursor_pointer()
            .px_2()
            .rounded_sm()
            .child(self.text);

        let elem = if let Some(handler) = self.on_click {
            elem.on_mouse_down(MouseButton::Left, move |event, window, app| (handler)(event, window, app))
        } else {
            elem
        };

        return match self.theme {
            ButtonTheme::Primary => elem
                .bg(rgb(PRIMARY_BG))
                .hover(|s| s.bg(rgb(PRIMARY_BG_HOVER)))
                .text_color(rgb(PRIMARY_TEXT)),
            ButtonTheme::Secondary => elem
                .bg(rgb(SECONDARY_BG))
                .hover(|s| s.bg(rgb(SECONDARY_BG_HOVER)))
                .text_color(rgb(SECONDARY_TEXT)),
        }
    }
}
