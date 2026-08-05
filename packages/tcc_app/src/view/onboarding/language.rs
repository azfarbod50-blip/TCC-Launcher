//! Onboarding language view

use freya::prelude::*;
use crate::view::onboarding::{onboarding_illustration, onboarding_page, onboarding_nav, step_heading};
use crate::components::{Button, Dropdown, Icon, IconType};
use crate::routes::Route;
use crate::theme::colors;

#[derive(PartialEq)]
pub struct OnboardingLanguage;

impl Component for OnboardingLanguage {
    fn render(&self) -> impl IntoElement {
        let languages = vec![
            ("English".to_string(), "en"),
            ("Spanish".to_string(), "es"),
            ("French".to_string(), "fr"),
            ("German".to_string(), "de"),
            ("Russian".to_string(), "ru"),
            ("Chinese".to_string(), "zh"),
        ];
        
        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Language",
                "Select your preferred language.",
            ))
            .child(
                Dropdown::new(languages)
                    .placeholder("English")
            )
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::Globe01),
            content,
            onboarding_nav(Some(Route::OnboardingMigration {}), Route::OnboardingAccount {}, true),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}