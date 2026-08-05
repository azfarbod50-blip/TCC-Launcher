//! Cluster shell layout

use freya::prelude::*;

#[derive(PartialEq)]
pub struct ClusterShell;

impl Component for ClusterShell {
    fn render(&self) -> impl IntoElement {
        freya::router::Outlet::<crate::routes::Route> {}
    }
}