//! 📐 LaTeX / Typst Renderer Component
//!
//! Strategy A: `typst` compiles Typst markup → `typst-render` → bitmap → GPUI.
//! Strategy B: `katex` renders LaTeX math → SVG → `resvg` → bitmap → GPUI.

use gpui::*;
use image::RgbaImage;
use std::sync::{Arc, Mutex};

use crate::bitmap_utils;

pub struct LatexView {
    source_code: String,
    rendered: Arc<Mutex<Option<RgbaImage>>>,
    mode: RenderMode,
    error_msg: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum RenderMode {
    Typst,
    KatexMath,
}

impl LatexView {
    pub fn new(_window: &Window, cx: &mut Context<Self>) -> Self {
        let default_typst = r#"
= Hello from Typst!

This is rendered *entirely in Rust* — no LaTeX installation needed.

$ integral_0^infinity e^(-x^2) d x = sqrt(pi) / 2 $

#table(
  columns: 3,
  [*Name*], [*Value*], [*Unit*],
  [Speed],  [299792458], [m/s],
  [Mass],   [9.109e-31], [kg],
)
"#
        .to_string();

        let mut view = Self {
            source_code: default_typst,
            rendered: Arc::new(Mutex::new(None)),
            mode: RenderMode::Typst,
            error_msg: None,
        };

        view.re_render(cx);
        view
    }

    fn re_render(&mut self, cx: &mut Context<Self>) {
        let source = self.source_code.clone();
        let rendered = self.rendered.clone();
        let mode = self.mode;

        cx.spawn(|this, mut cx| async move {
            let result = tokio::task::spawn_blocking(move || match mode {
                RenderMode::Typst => render_typst(&source),
                RenderMode::KatexMath => render_katex_math(&source),
            })
            .await
            .unwrap();

            match result {
                Ok(img) => {
                    *rendered.lock().unwrap() = Some(img);
                    cx.update(|cx| {
                        this.update(cx, |view, cx| {
                            view.error_msg = None;
                            cx.notify();
                        })
                    }).ok();
                }
                Err(e) => {
                    cx.update(|cx| {
                        this.update(cx, |view, cx| {
                            view.error_msg = Some(e.to_string());
                            cx.notify();
                        })
                    }).ok();
                }
            }
        })
        .detach();
    }
}

impl Render for LatexView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let rendered = self.rendered.lock().unwrap();

        div()
            .v_flex()
            .gap_3()
            .p_4()
            .size_full()
            .child(
                div()
                    .h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        div()
                            .text_color(rgb(0xcdd6f4))
                            .text_xl()
                            .child("📐 LaTeX / Typst Renderer"),
                    )
                    // Mode toggle
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .bg(if self.mode == RenderMode::Typst {
                                rgb(0x89b4fa)
                            } else {
                                rgb(0x585b70)
                            })
                            .text_color(rgb(0x1e1e2e))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child("Typst"),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .bg(if self.mode == RenderMode::KatexMath {
                                rgb(0x89b4fa)
                            } else {
                                rgb(0x585b70)
                            })
                            .text_color(rgb(0x1e1e2e))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child("KaTeX Math"),
                    ),
            )
            .child(
                // Split view: Source | Rendered
                div()
                    .h_flex()
                    .gap_4()
                    .flex_1()
                    // Source editor (simple text display for now)
                    .child(
                        div()
                            .v_flex()
                            .flex_1()
                            .child(
                                div()
                                    .text_color(rgb(0xa6adc8))
                                    .text_sm()
                                    .pb_1()
                                    .child("Source:"),
                            )
                            .child(
                                div()
                                    .p_3()
                                    .bg(rgb(0x181825))
                                    .rounded(px(6.))
                                    .text_color(rgb(0xa6e3a1))
                                    .text_sm()
                                    .overflow_y_scroll()
                                    .max_h(px(500.))
                                    .child(self.source_code.clone()),
                            ),
                    )
                    // Rendered output
                    .child(
                        div()
                            .v_flex()
                            .flex_1()
                            .child(
                                div()
                                    .text_color(rgb(0xa6adc8))
                                    .text_sm()
                                    .pb_1()
                                    .child("Output:"),
                            )
                            .child(
                                div()
                                    .p_3()
                                    .bg(rgb(0xffffff))
                                    .rounded(px(6.))
                                    .min_h(px(400.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        if let Some(ref err) = self.error_msg {
                                            div()
                                                .text_color(rgb(0xf38ba8))
                                                .child(format!("Error: {}", err))
                                        } else if let Some(ref img_data) = *rendered {
                                            let source =
                                                bitmap_utils::rgba_to_gpui_image(img_data);
                                            div().child(
                                                img(source)
                                                    .w_full()
                                                    .object_fit(ObjectFit::Contain),
                                            )
                                        } else {
                                            div()
                                                .text_color(rgb(0x6c7086))
                                                .child("Compiling...")
                                        },
                                    ),
                            ),
                    ),
            )
    }
}

/// Render Typst source to an RGBA image using typst + typst-render (pure Rust)
fn render_typst(source: &str) -> anyhow::Result<RgbaImage> {
    use typst::foundations::Smart;

    // Create a minimal Typst world (fonts, file system)
    // In production you'd implement the `World` trait properly
    let world = create_typst_world(source)?;

    // Compile the Typst document
    let document = typst::compile(&world)
        .output
        .map_err(|e| anyhow::anyhow!("Typst compile error: {:?}", e))?;

    // Render first page to a pixmap
    let page = document
        .pages
        .first()
        .ok_or_else(|| anyhow::anyhow!("No pages in document"))?;

    let pixel_per_pt = 2.0; // 2x resolution for crisp rendering
    let pixmap = typst_render::render(page, pixel_per_pt);

    // Convert tiny-skia Pixmap → image::RgbaImage
    let width = pixmap.width();
    let height = pixmap.height();
    let rgba_data = pixmap.data().to_vec(); // already RGBA premultiplied

    RgbaImage::from_raw(width, height, rgba_data)
        .ok_or_else(|| anyhow::anyhow!("Failed to create RGBA image from typst render"))
}

/// Render LaTeX math expression to RGBA via KaTeX → SVG → resvg
fn render_katex_math(latex_expr: &str) -> anyhow::Result<RgbaImage> {
    // 1. Render LaTeX math → SVG string using katex
    let opts = katex::Opts::builder()
        .display_mode(true)
        .build()
        .map_err(|e| anyhow::anyhow!("KaTeX opts error: {:?}", e))?;

    let html = katex::render_with_opts(latex_expr, &opts)
        .map_err(|e| anyhow::anyhow!("KaTeX render error: {:?}", e))?;

    // 2. KaTeX outputs HTML with embedded SVG — extract or wrap in SVG
    let svg_string = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="600" height="200">
            <foreignObject width="100%" height="100%">
                <div xmlns="http://www.w3.org/1999/xhtml"
                     style="font-size:24px; padding:20px;">
                    {}
                </div>
            </foreignObject>
        </svg>"#,
        html
    );

    // 3. Render SVG → RGBA using resvg (pure Rust)
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(&svg_string, &opt)?;

    let size = tree.size();
    let width = size.width() as u32;
    let height = size.height() as u32;

    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(width, height)
            .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;

    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    RgbaImage::from_raw(width, height, pixmap.data().to_vec())
        .ok_or_else(|| anyhow::anyhow!("Failed to create RGBA image"))
}

// Placeholder — in production, implement the full typst::World trait
fn create_typst_world(_source: &str) -> anyhow::Result<impl typst::World> {
    // You would create a struct that implements typst::World
    // providing: source files, fonts, current date, etc.
    // See typst-as-lib crate for a ready-made implementation
    todo!("Implement typst::World — use typst-as-lib crate for convenience")
}
