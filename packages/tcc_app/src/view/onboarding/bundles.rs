//! Onboarding bundles view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingBundles;

impl Component for OnboardingBundles {
    fn render(&self) -> impl IntoElement {
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Modpacks",
                "Choose modpacks to install (optional).",
            ))
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::Download),
            content,
            onboarding_nav(Some(Route::OnboardingPreferences {}), Route::OnboardingDownloading {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}