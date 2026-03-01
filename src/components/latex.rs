//! 📐 LaTeX / Typesetting Panel
//!
//! Stack: typst + typst-render (full typesetting) + katex (math only)
//! Pipeline: Source → typst compiler → typst-render → bitmap → GPUI
//!       OR: LaTeX math → katex → SVG → resvg → bitmap → GPUI

use gpui::*;
use super::video::card;

pub struct LatexPanel {
    status: String,
    features: Vec<&'static str>,
}

impl LatexPanel {
    pub fn new() -> Self {
        Self {
            status: "Ready".to_string(),
            features: vec![
                "typst — full typesetting compiler (pure Rust)",
                "typst-render — typst → bitmap PNG (pure Rust)",
                "typst-as-lib — easy library wrapper",
                "katex — LaTeX math → HTML/SVG",
                "resvg — SVG → bitmap fallback (pure Rust)",
                "100% Rust — no tectonic/XeTeX C deps",
            ],
        }
    }

    pub fn render_card(&self) -> impl IntoElement {
        let items = self
            .features
            .iter()
            .map(|f| {
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0xBBBBBB))
                    .child(format!("  📐 {}", f))
            })
            .collect::<Vec<_>>();

        card(
            "📐 LaTeX / Typesetting",
            &self.status,
            rgb(0x1abc9c),
            "Typst compiler → bitmap → GPUI element",
            items,
        )
    }
}
