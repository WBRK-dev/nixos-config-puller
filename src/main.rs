use gpui::{AppContext, Application, Bounds, Point, Size, WindowBounds, WindowOptions, px};

use crate::{nixos::configuration::check_for_updates, ui::pages::RootView};

mod config;
mod nixos;
mod ui;

fn main() {
    let updates = match check_for_updates() {
        nixos::configuration::CheckForUpdatesResult::UpdateAvailable(result) => result,
        nixos::configuration::CheckForUpdatesResult::NoUpdateAvailable => {
            println!("No update available, exiting...");
            return;
        }
    };

    Application::new().run(|app| {
        app.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Point::new(px(100.0), px(100.0)),
                    Size::new(px(600.0), px(400.0)),
                ))),
                is_resizable: false,
                ..WindowOptions::default()
            },
            |_window, app| {
                app.new(|_| RootView::new(updates))
            }
        ).unwrap();
    })
}
