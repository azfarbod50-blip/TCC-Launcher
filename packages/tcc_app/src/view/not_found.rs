//! Not found view

use freya::prelude::*;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct NotFound {
    pub path: Vec<String>,
}

impl Component for NotFound {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .cross_align(Alignment::Center)
            .child(
                label()
                    .text("404 - Page Not Found")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
            .child(
                label()
                    .text(format!("Path: {}", self.path.join("/")))
                    .font_size(14.)
                    .color(colors::fg_secondary()),
            )
    }
}