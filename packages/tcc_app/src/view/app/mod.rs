//! App views module

pub mod browser;
pub mod cluster;
pub mod settings;

pub use browser::{Browser, BrowserPackage};
pub use cluster::{
    ClusterLogs, ClusterMods, ClusterOverview, ClusterScreenshots, ClusterSettings,
    ClusterShaders, ClusterTextures, ProcessLogs,
};
pub use settings::{
    SettingsApis, SettingsAppearance, SettingsChangelog, SettingsDeveloper, SettingsJava,
    SettingsLanguage, SettingsLauncher, SettingsMinecraft, SettingsStorage,
};

use freya::prelude::*;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct Home;

impl Component for Home {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text("Welcome to TCC Launcher")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct Clusters;

impl Component for Clusters {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text("Versions")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct Accounts;

impl Component for Accounts {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text("Accounts")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct AccountSkins;

impl Component for AccountSkins {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text("Skins")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct Stats;

impl Component for Stats {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text("Statistics")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}

#[derive(PartialEq)]
pub struct Debug;

impl Component for Debug {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(Gaps::new_all(24.))
            .child(
                label()
                    .text("Debug")
                    .font_size(24.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
    }
}