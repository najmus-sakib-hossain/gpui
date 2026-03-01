//! 🏗️ GPUI Single-Binary Desktop App
//!
//! A pure-Rust multimedia powerhouse showcasing:
//!   🎬 Video   🧊 3D   🔊 Audio   📄 PDF   📝 Docs   📐 LaTeX   📊 Charts
//!
//! Rule: The user downloads ONE binary. Nothing else. Ever.

mod components;

use gpui::*;
use components::homepage::Homepage;

fn main() {
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(100.0), px(100.0)),
                    size: size(px(1280.0), px(900.0)),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("🏗️ GPUI Desktop — Single Binary Multimedia".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new_view(|_cx| Homepage::new()),
        )
        .unwrap();
    });
}
