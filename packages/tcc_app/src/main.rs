//! TCC Launcher Freya App

mod assets;
mod components;
mod constants;
mod events;
mod hooks;
mod install;
mod launcher;
mod layout;
mod main;
mod notifications;
mod platform;
mod routes;
mod state;
mod theme;
mod transfer;
mod ui;
mod updater;
mod utils;
mod view;

use freya::prelude::*;

fn main() {
    launch(cfg!(debug_assertions));
}

fn launch(devtools: bool) {
    let config = LaunchConfig::default()
        .with_title("TCC Launcher")
        .with_size((1200.0, 800.0))
        .with_devtools(devtools);
    
    launch_with_props(app, config, ()).unwrap();
}

fn app() -> Element {
    use crate::hooks::use_provide_actions;
    use crate::state::LauncherInit;
    
    let actions = crate::hooks::Actions::new();
    use_provide_actions(&actions);
    
    // Initialize the launcher
    let init = LauncherInit::new();
    
    rsx! {
        freya::router::Router { crate::router() }
    }
}