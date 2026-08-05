//! Cluster views

use freya::prelude::*;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct ClusterOverview {
    pub cluster_id: i64,
}

impl Component for ClusterOverview {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Overview: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ClusterLogs {
    pub cluster_id: i64,
}

impl Component for ClusterLogs {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Logs: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ProcessLogs {
    pub cluster_id: i64,
}

impl Component for ProcessLogs {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Process Logs: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ClusterScreenshots {
    pub cluster_id: i64,
}

impl Component for ClusterScreenshots {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Screenshots: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ClusterMods {
    pub cluster_id: i64,
}

impl Component for ClusterMods {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Mods: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ClusterShaders {
    pub cluster_id: i64,
}

impl Component for ClusterShaders {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Shaders: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ClusterTextures {
    pub cluster_id: i64,
}

impl Component for ClusterTextures {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Textures: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct ClusterSettings {
    pub cluster_id: i64,
}

impl Component for ClusterSettings {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text(format!("Cluster Settings: {}", self.cluster_id))
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}