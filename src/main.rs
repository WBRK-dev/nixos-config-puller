use gpui::{AppContext, Application, Bounds, Point, Size, WindowBounds, WindowOptions, px};

use crate::ui::pages::RootView;

mod ui;

fn main() {
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
                app.new(|_| RootView::new(String::new()))
            }
        ).unwrap();
    })
}
