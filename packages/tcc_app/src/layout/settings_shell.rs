//! Settings shell layout

use freya::prelude::*;

#[derive(PartialEq)]
pub struct SettingsShell;

impl Component for SettingsShell {
    fn render(&self) -> impl IntoElement {
        freya::router::Outlet::<crate::routes::Route> {}
    }
}