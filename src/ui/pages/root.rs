use std::{io::{BufRead, BufReader}, process::{Command, Stdio}, sync::mpsc};

use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, white};

use crate::ui::components::atoms::{ButtonTheme, TerminalView, button};

pub struct RootView {
    terminal: TerminalView,
}

impl RootView {
    pub fn new(buffer: String) -> Self {
        Self {
            terminal: TerminalView::new(buffer),
        }
    }

    fn run_command(&mut self, cx: &mut Context<Self>) {
        let (tx, rx) = mpsc::channel::<String>();

        std::thread::spawn(move || {
            let mut child = Command::new("sh")
                .args(["-c", "echo starting; ls -la; sleep 2; echo done"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to spawn command");

            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let line = line.unwrap_or_default();
                if tx.send(line).is_err() {
                    break;
                }
            }
        });

        cx.spawn(async move |root_view, cx| {
            loop {
                match rx.try_recv() {
                    Ok(line) => {
                        let _ = root_view.update(cx, |view, cx| {
                            view.terminal.append(&line);
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
            view.run_command(cx);
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
