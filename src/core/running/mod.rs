mod game;
mod main_menu;

use bevy::prelude::*;

use crate::ui;

use super::AppState;

pub fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<RunningState>();
    app.enable_state_scoped_entities::<RunningState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<RunningState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(Startup, setup);

    // Sub plugins
    app.add_plugins((ui::plugin, main_menu::plugin, game::plugin));
}

/// Core state of the application.
#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(AppState = AppState::Running)]
pub enum RunningState {
    #[default]
    Menu,
    Game,
}

/// Setup the main camera.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
