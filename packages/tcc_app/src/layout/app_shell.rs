//! App shell layout

use freya::prelude::*;
use crate::components::{AccountSwitcher, Button, Icon, IconType};

#[derive(PartialEq)]
pub struct AppShell;

impl Component for AppShell {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                freya::router::Outlet::<crate::routes::Route> {}
            )
            .child(AccountSwitcher)
    }
}