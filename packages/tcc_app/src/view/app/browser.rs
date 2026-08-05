//! Browser views

use freya::prelude::*;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct Browser {
    pub cluster_id: i64,
    pub package_type: String,
    pub pick_cluster: bool,
}

impl Component for Browser {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Browser - Cluster: {}, Type: {}", self.cluster_id, self.package_type))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct BrowserPackage {
    pub cluster_id: i64,
    pub package_type: String,
    pub package_id: String,
}

impl Component for BrowserPackage {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Package: {}", self.package_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}