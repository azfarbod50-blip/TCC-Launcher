//! Root layout

use freya::prelude::*;

#[derive(PartialEq)]
pub struct RootLayout;

impl Component for RootLayout {
    fn render(&self) -> impl IntoElement {
        rsx! {
            freya::router::Outlet::<crate::routes::Route> {}
        }
    }
}