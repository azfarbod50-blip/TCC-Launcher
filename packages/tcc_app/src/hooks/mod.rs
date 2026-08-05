//! Hooks for TCC Launcher

mod actions;
mod queries;
mod view_state;

pub use actions::{Actions, NotificationBuilder, PumpSignal};
pub use queries::*;
pub use view_state::{PersistedView, use_view_state};

use crate::notifications::NotificationSnapshot;
use crate::state::{
    AppChannel, GameState, InstallState, LauncherInit, LoginProgress, SettingsState,
};
use freya::prelude::*;
use freya::radio::use_radio;

/// Publishes the actions handle so components can reach it without prop
/// drilling. Provided once, at the root.
pub fn use_provide_actions(actions: &Actions) {
    let actions = actions.clone();
    use_provide_root_context(move || actions.clone());
}

pub fn use_dispatch() -> Actions {
    consume_root_context::<Actions>()
}

/// Subscribes to one concern of the app state.
///
/// Each of these wakes its component only when *that* channel is written, so a
/// toast timer tick does not re-render a component that reads only `data_dir`.
pub fn use_launcher() -> LauncherInit {
    use_radio(AppChannel::Launcher).read().launcher.clone()
}

pub fn use_settings_snapshot() -> SettingsState {
    use_radio(AppChannel::Settings).read().settings.clone()
}

/// Derives the render view from the engine.
///
/// Built on read rather than published on write: during a download the engine
/// changes tens of thousands of times, and cloning the inbox each time was the
/// snapshot channel's main cost.
pub fn use_notifications_snapshot() -> NotificationSnapshot {
    let radio = use_radio(AppChannel::Notifications);
    let state = radio.read();
    state.notifications.snapshot(
        &state.inbox,
        state.center_open,
        crate::events::prompt_view(&state),
    )
}

pub fn use_account_switcher_open() -> bool {
    use_radio(AppChannel::AccountSwitcher)
        .read()
        .account_switcher_open
}

pub fn use_game_snapshot() -> GameState {
    use_radio(AppChannel::Game).read().game.clone()
}

pub fn use_installs_snapshot() -> InstallState {
    use_radio(AppChannel::Installs).read().installs.clone()
}

pub fn use_offline_login_status() -> Option<LoginProgress> {
    None // No Microsoft login in TCC
}