//! Onboarding welcome view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingWelcome;

impl Component for OnboardingWelcome {
    fn render(&self) -> impl IntoElement {
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Welcome to TCC Launcher",
                "An offline Minecraft launcher - no Microsoft account required.",
            ))
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::Home),
            content,
            onboarding_nav(None, Route::OnboardingTerms {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}