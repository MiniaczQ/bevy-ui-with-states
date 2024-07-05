mod game;
mod menu;

use bevy::prelude::*;

use crate::ui;

pub fn plugin(app: &mut App) {
    // State setup
    app.init_state::<RunningState>();
    app.enable_state_scoped_entities::<RunningState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<RunningState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(Startup, setup);

    // Sub plugins
    app.add_plugins((ui::plugin, menu::plugin, game::plugin));
}

/// Core state of the application.
#[derive(States, Debug, PartialEq, Hash, Eq, Clone, Default)]
pub enum RunningState {
    #[default]
    Menu,
    Game,
}

/// Setup the main camera.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
