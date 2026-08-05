//! Onboarding preferences view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingPreferences;

impl Component for OnboardingPreferences {
    fn render(&self) -> impl IntoElement {
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Accessibility",
                "Configure accessibility options.",
            ))
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::Settings),
            content,
            onboarding_nav(Some(Route::OnboardingLanguage {}), Route::OnboardingBundles {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}