//! 📄 PDF Viewer Component
//!
//! Strategy: `hayro` renders PDF pages to PNG bitmaps → GPUI image element.
//! 100% pure Rust. No PDFium. No C deps.

use gpui::*;
use gpui_component::StyledExt;
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
        cx.spawn(async move |this: WeakEntity<PdfView>, cx| {
            let rendered = smol::unblock(move || {
                render_pdf_pages(&path)
            }).await;

            if let Ok(rendered_pages) = rendered {
                let count = rendered_pages.len();
                *pages_clone.lock().unwrap() = rendered_pages;
                cx.update(|cx| {
                    this.update(cx, |view, cx: &mut Context<PdfView>| {
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
        let prev = cx.listener(|this, _: &MouseDownEvent, _, cx| this.prev_page(cx));
        let next = cx.listener(|this, _: &MouseDownEvent, _, cx| this.next_page(cx));

        div()
            .v_flex()
            .gap_3()
            .items_center()
            .p_4()
            .child(
                div()
                    .text_color(rgb(0xcdd6f4))
                    .text_xl()
                    .child("📄 PDF Viewer (hayro — pure Rust)"),
            )
            .child(
                // File info
                div()
                    .text_color(rgb(0x6c7086))
                    .text_sm()
                    .child(if self.file_path.is_empty() {
                        "No file loaded — pass a .pdf path".to_string()
                    } else {
                        self.file_path.clone()
                    }),
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
                            .on_mouse_down(MouseButton::Left, prev),
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
                            .on_mouse_down(MouseButton::Left, next),
                    ),
            )
    }
}

/// Render all pages of a PDF to RGBA images using hayro (pure Rust)
fn render_pdf_pages(path: &str) -> anyhow::Result<Vec<RgbaImage>> {
    let pdf_data = std::fs::read(path)?;
    let pdf = hayro::Pdf::new(std::sync::Arc::new(pdf_data))
        .map_err(|e| anyhow::anyhow!("Failed to parse PDF: {:?}", e))?;
    let mut pages = Vec::new();
    let interp = hayro::InterpreterSettings::default();
    let render_settings = hayro::RenderSettings::default();
    for page in pdf.pages().iter() {
        let pixmap = hayro::render(page, &interp, &render_settings);
        let width = pixmap.width() as u32;
        let height = pixmap.height() as u32;
        let rgba_data = pixmap.take_u8();
        if let Some(img) = RgbaImage::from_raw(width, height, rgba_data) {
            pages.push(img);
        }
    }
    Ok(pages)
}
