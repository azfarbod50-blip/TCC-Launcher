//! Onboarding migration view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingMigration;

impl Component for OnboardingMigration {
    fn render(&self) -> impl IntoElement {
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Migration",
                "No migration needed for offline accounts.",
            ))
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::Folder),
            content,
            onboarding_nav(Some(Route::OnboardingTerms {}), Route::OnboardingLanguage {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}