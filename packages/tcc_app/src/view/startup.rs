//! Startup view

use freya::prelude::*;
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;
use crate::hooks::use_dispatch;
use freya::router::RouterContext;

#[derive(PartialEq)]
pub struct Startup;

impl Component for Startup {
    fn render(&self) -> impl IntoElement {
        let dispatch = use_dispatch();
        
        let navigate_to_onboarding = move |_| {
            let _ = RouterContext::get().push(Route::OnboardingWelcome {});
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background(colors::page_elevated())
            .cross_align(Alignment::Center)
            .spacing(32.)
            .child(
                Icon::new(IconType::Brand)
                    .size(120.)
                    .color(colors::brand()),
            )
            .child(
                label()
                    .text("TCC Launcher")
                    .font_size(32.)
                    .font_weight(FontWeight::BOLD)
                    .color(colors::fg_primary()),
            )
            .child(
                label()
                    .text("Offline Minecraft Launcher")
                    .font_size(16.)
                    .color(colors::fg_secondary()),
            )
            .child(
                Button::new()
                    .primary()
                    .large()
                    .on_press(navigate_to_onboarding)
                    .child(Icon::new(IconType::Play).size(20.))
                    .text("Get Started"),
            )
    }
}