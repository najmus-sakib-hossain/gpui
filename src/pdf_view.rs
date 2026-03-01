//! 📄 PDF Viewer Component
//!
//! Strategy: `hayro` renders PDF pages to PNG bitmaps → GPUI image element.
//! 100% pure Rust. No PDFium. No C deps. Compiles right into your binary.

use gpui::*;
use image::RgbaImage;
use std::sync::{Arc, Mutex};

use crate::bitmap_utils;

pub struct PdfView {
    pages: Arc<Mutex<Vec<RgbaImage>>>,
    current_page: usize,
    total_pages: usize,
    loaded: bool,
    file_path: String,
}

impl PdfView {
    pub fn new(file_path: &str, _window: &Window, cx: &mut Context<Self>) -> Self {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let pages_clone = pages.clone();
        let path = file_path.to_string();

        // Render PDF pages in background
        cx.spawn(|this, mut cx| async move {
            let rendered = tokio::task::spawn_blocking(move || {
                render_pdf_pages(&path)
            })
            .await
            .unwrap();

            if let Ok(rendered_pages) = rendered {
                let count = rendered_pages.len();
                *pages_clone.lock().unwrap() = rendered_pages;
                cx.update(|cx| {
                    this.update(cx, |view, cx| {
                        view.total_pages = count;
                        view.loaded = true;
                        cx.notify();
                    })
                }).ok();
            }
        })
        .detach();

        Self {
            pages,
            current_page: 0,
            total_pages: 0,
            loaded: false,
            file_path: file_path.to_string(),
        }
    }

    fn next_page(&mut self, cx: &mut Context<Self>) {
        if self.current_page + 1 < self.total_pages {
            self.current_page += 1;
            cx.notify();
        }
    }

    fn prev_page(&mut self, cx: &mut Context<Self>) {
        if self.current_page > 0 {
            self.current_page -= 1;
            cx.notify();
        }
    }
}

impl Render for PdfView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pages = self.pages.lock().unwrap();

        div()
            .v_flex()
            .gap_3()
            .items_center()
            .p_4()
            .child(
                div()
                    .text_color(rgb(0xcdd6f4))
                    .text_xl()
                    .child("📄 PDF Viewer (hayro \u2014 pure Rust)"),
            )
            .child(
                // PDF page display
                div()
                    .w(px(620.))
                    .min_h(px(800.))
                    .bg(rgb(0xffffff))
                    .rounded(px(4.))
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(if let Some(page_img) = pages.get(self.current_page) {
                        let source = bitmap_utils::rgba_to_gpui_image(page_img);
                        div().child(
                            img(source)
                                .w(px(600.))
                                .object_fit(ObjectFit::Contain),
                        )
                    } else {
                        div()
                            .text_color(rgb(0x6c7086))
                            .child(if self.loaded {
                                "No pages to display"
                            } else {
                                "Rendering PDF..."
                            })
                    }),
            )
            // Page navigation
            .child(
                div()
                    .h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .bg(rgb(0x585b70))
                            .text_color(rgb(0xcdd6f4))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child("◀ Prev")
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.notify();
                            }),
                    )
                    .child(
                        div()
                            .text_color(rgb(0xa6adc8))
                            .child(format!(
                                "Page {} of {}",
                                self.current_page + 1,
                                self.total_pages
                            )),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .bg(rgb(0x585b70))
                            .text_color(rgb(0xcdd6f4))
                            .rounded(px(6.))
                            .cursor_pointer()
                            .child("Next ▶")
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.notify();
                            }),
                    ),
            )
    }
}

/// Render all pages of a PDF to RGBA images using hayro (pure Rust)
fn render_pdf_pages(path: &str) -> anyhow::Result<Vec<RgbaImage>> {
    let pdf_data = std::fs::read(path)?;
    let mut pages = Vec::new();

    // hayro renders PDF pages to PNG bytes
    // Configure rendering DPI (150 gives good quality)
    let dpi = 150.0;

    // Use hayro to interpret the PDF and render each page
    let document = hayro::Document::parse(&pdf_data)?;
    let page_count = document.page_count();

    for page_idx in 0..page_count {
        // Render page to PNG bytes
        let png_bytes = document.render_page(page_idx, dpi)?;

        // Decode PNG → RGBA
        let dyn_img = image::load_from_memory(&png_bytes)?;
        pages.push(dyn_img.to_rgba8());
    }

    Ok(pages)
}
