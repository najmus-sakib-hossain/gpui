//! 📝 Document Viewer Component
//!
//! Strategy: `docx-rs` parses DOCX → extracts text/styles → renders with GPUI div elements.
//! Pipeline: Parse → extract text/styles/images → custom GPUI layout elements

use docx_rs::*;
use gpui::*;
use std::io::Read;

pub struct DocView {
    paragraphs: Vec<DocParagraph>,
    file_path: String,
    loaded: bool,
}

#[derive(Clone)]
struct DocParagraph {
    text: String,
    is_heading: bool,
    is_bold: bool,
    font_size: f32,
}

impl DocView {
    pub fn new(file_path: &str, _window: &Window, _cx: &mut Context<Self>) -> Self {
        let path = file_path.to_string();
        let mut view = Self {
            paragraphs: Vec::new(),
            file_path: path.clone(),
            loaded: false,
        };

        // Parse DOCX synchronously (fast enough for most docs)
        if let Ok(paragraphs) = parse_docx(&path) {
            view.paragraphs = paragraphs;
            view.loaded = true;
        }

        view
    }
}

impl Render for DocView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_3()
            .p_4()
            .size_full()
            .child(
                div()
                    .text_color(rgb(0xcdd6f4))
                    .text_xl()
                    .child("📝 Document Viewer (docx-rs \u2014 pure Rust)"),
            )
            .child(
                // Document content area
                div()
                    .v_flex()
                    .gap_2()
                    .p_6()
                    .w(px(700.))
                    .bg(rgb(0xffffff))
                    .rounded(px(8.))
                    .shadow_lg()
                    .overflow_y_scroll()
                    .max_h(px(600.))
                    .children(
                        self.paragraphs
                            .iter()
                            .map(|para| {
                                let mut el = div()
                                    .w_full()
                                    .text_color(rgb(0x1e1e2e));

                                if para.is_heading {
                                    el = el
                                        .text_xl()
                                        .pb_2()
                                        .border_b_1()
                                        .border_color(rgb(0xcccccc));
                                } else {
                                    el = el.text_sm();
                                }

                                el.child(para.text.clone())
                            })
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}

/// Parse a DOCX file and extract paragraphs
fn parse_docx(path: &str) -> anyhow::Result<Vec<DocParagraph>> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let doc = docx_rs::read_docx(&buf)?;
    let mut paragraphs = Vec::new();

    for child in doc.document.children {
        match child {
            DocumentChild::Paragraph(para) => {
                let mut text = String::new();
                let mut is_bold = false;
                let mut is_heading = false;
                let mut font_size = 12.0;

                // Check paragraph style for headings
                if let Some(ref style) = para.property.style {
                    if style.val.starts_with("Heading") {
                        is_heading = true;
                        font_size = 18.0;
                    }
                }

                // Extract text from runs
                for content in &para.children {
                    match content {
                        ParagraphChild::Run(run) => {
                            // Check run properties
                            if let Some(ref rp) = run.run_property {
                                if rp.bold.is_some() {
                                    is_bold = true;
                                }
                            }
                            for run_child in &run.children {
                                match run_child {
                                    RunChild::Text(t) => {
                                        text.push_str(&t.text);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if !text.is_empty() {
                    paragraphs.push(DocParagraph {
                        text,
                        is_heading,
                        is_bold,
                        font_size,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(paragraphs)
}
