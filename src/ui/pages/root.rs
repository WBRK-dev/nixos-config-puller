use std::sync::mpsc;

use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, white};

use crate::{nixos::configuration::upgrade, ui::components::{atoms::{ButtonTheme, button}, organisms::{TerminalLine, TerminalView}}};

pub struct RootView {
    terminal: TerminalView,
}

impl RootView {
    pub fn new(buffer: Vec<TerminalLine>) -> Self {
        Self {
            terminal: TerminalView::new(buffer),
        }
    }

    fn upgrade_configuration(&mut self, cx: &mut Context<Self>) {
        let (tx, rx) = mpsc::channel::<TerminalLine>();

        std::thread::spawn(move || {
            upgrade(tx);
        });

        cx.spawn(async move |root_view, cx| {
            loop {
                match rx.try_recv() {
                    Ok(line) => {
                        let _ = root_view.update(cx, |view, cx| {
                            view.terminal.append(line);
                            cx.notify();
                        });
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
        }).detach();
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let on_upgrade = cx.listener(|view: &mut Self, _event, _window, cx| {
            view.upgrade_configuration(cx);
        });

        div()
            .size_full()
            .bg(white())
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().flex_1().w_full().overflow_hidden().child(self.terminal.clone()))
            .child(
                div()
                    .w_full()
                    .flex()
                    .justify_end()
                    .gap_2()
                    .children([
                        button()
                            .text("Ignore")
                            .theme(ButtonTheme::Secondary),
                        button()
                            .text("Upgrade")
                            .theme(ButtonTheme::Primary)
                            .on_click(on_upgrade),
                    ])
            )
    }
}
