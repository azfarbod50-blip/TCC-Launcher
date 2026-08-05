//! Onboarding terms view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingTerms;

impl Component for OnboardingTerms {
    fn render(&self) -> impl IntoElement {
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Terms & Privacy",
                "By continuing, you agree to our terms of service and privacy policy.",
            ))
            .child(
                label()
                    .text("TCC Launcher is an open-source offline Minecraft launcher.\nNo data is collected or sent to any third parties.")
                    .font_size(14.)
                    .color(colors::fg_secondary()),
            )
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::File),
            content,
            onboarding_nav(Some(Route::OnboardingWelcome {}), Route::OnboardingMigration {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}