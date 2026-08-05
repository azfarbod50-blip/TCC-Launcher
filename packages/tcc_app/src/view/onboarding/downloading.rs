//! Onboarding downloading view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingDownloading;

impl Component for OnboardingDownloading {
    fn render(&self) -> impl IntoElement {
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Finishing",
                "Setting up your launcher...",
            ))
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::Loading02),
            content,
            onboarding_nav(Some(Route::OnboardingBundles {}), Route::Home {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}