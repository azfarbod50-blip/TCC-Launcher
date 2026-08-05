//! UI utilities

use freya::prelude::*;

pub fn border_all_color(width: f32, color: Color) -> Border {
    Border::new()
        .fill(color)
        .width(width)
        .alignment(BorderAlignment::Inner)
}

pub fn divider() -> impl IntoElement {
    crate::theme::colors::divider()
}