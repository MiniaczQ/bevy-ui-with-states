use bevy::prelude::*;

use crate::{game, menu, ui};

pub fn plugin(app: &mut App) {
    // State setup
    app.init_state::<CoreState>();
    app.enable_state_scoped_entities::<CoreState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<CoreState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(Startup, setup);

    // Sub plugins
    app.add_plugins((ui::plugin, menu::plugin, game::plugin));
}

/// Core state of the application.
#[derive(States, Debug, PartialEq, Hash, Eq, Clone, Default)]
pub enum CoreState {
    #[default]
    Menu,
    Game,
}

/// Setup the main camera.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
