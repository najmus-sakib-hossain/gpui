//! 🏗️ GPUI Single-Binary Desktop App
//!
//! A pure-Rust multimedia powerhouse showcasing:
//!   🎬 Video   🧊 3D   🔊 Audio   📄 PDF   📝 Docs   📐 LaTeX   📊 Charts
//!
//! Rule: The user downloads ONE binary. Nothing else. Ever.

use gpui::*;
use gpui_component::*;

mod bitmap_utils;
mod video_view;
mod three_d_view;
mod audio_view;
mod pdf_view;
mod doc_view;
mod latex_view;
mod chart_view;

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);
    app.run(move |cx| {
        gpui_component::init(cx);
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| AppRoot::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}

/// Top-level application view hosting all media components
struct AppRoot {
    current_tab: Tab,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Video,
    ThreeD,
    Audio,
    Pdf,
    Doc,
    Latex,
    Chart,
}

impl AppRoot {
    fn new(_window: &Window, _cx: &mut Context<Self>) -> Self {
        Self {
            current_tab: Tab::Chart, // default tab
        }
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tab = self.current_tab;

        div()
            .size_full()
            .v_flex()
            .bg(rgb(0x1e1e2e))
            .child(
                // ── Tab bar ──
                div()
                    .h_flex()
                    .gap_1()
                    .p_2()
                    .bg(rgb(0x181825))
                    .children(
                        [
                            ("🎬 Video",  Tab::Video),
                            ("🧊 3D",     Tab::ThreeD),
                            ("🔊 Audio",  Tab::Audio),
                            ("📄 PDF",    Tab::Pdf),
                            ("📝 Docs",   Tab::Doc),
                            ("📐 LaTeX",  Tab::Latex),
                            ("📊 Chart",  Tab::Chart),
                        ]
                        .into_iter()
                        .map(|(label, t)| {
                            let is_active = tab == t;
                            div()
                                .px_4()
                                .py_2()
                                .rounded(px(6.))
                                .cursor_pointer()
                                .bg(if is_active { rgb(0x585b70) } else { rgb(0x313244) })
                                .text_color(rgb(0xcdd6f4))
                                .child(label)
                                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                    cx.notify();
                                })
                        }),
                    ),
            )
            .child(
                // ── Content area ──
                div()
                    .flex_1()
                    .p_4()
                    .overflow_y_scroll()
                    .child(match self.current_tab {
                        Tab::Video  => div().child("🎬 Video Player \u2014 symphonia + openh264 H.264 decoder with frame-by-frame playback"),
                        Tab::ThreeD => div().child("🧊 3D Renderer \u2014 wgpu off-screen colored cube with WGSL shaders + rotation controls"),
                        Tab::Audio  => div().child("🔊 Audio Player \u2014 rodio + symphonia decode & playback with volume bar"),
                        Tab::Pdf    => div().child("📄 PDF Viewer \u2014 hayro pure Rust renderer with page-by-page navigation"),
                        Tab::Doc    => div().child("📝 Document Viewer \u2014 docx-rs DOCX parser with heading/bold extraction"),
                        Tab::Latex  => div().child("📐 LaTeX / Typst Renderer \u2014 typst compiler + katex math, split source/output"),
                        Tab::Chart  => div().child("📊 Chart Renderer \u2014 plotters line/bar/scatter/area to in-memory bitmap"),
                    }),
            )
    }
}
