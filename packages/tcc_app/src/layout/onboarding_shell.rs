//! Onboarding shell layout

use freya::prelude::*;

#[derive(PartialEq)]
pub struct OnboardingShell;

impl Component for OnboardingShell {
    fn render(&self) -> impl IntoElement {
        freya::router::Outlet::<crate::routes::Route> {}
    }
}