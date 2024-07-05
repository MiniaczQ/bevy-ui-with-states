mod loading;
mod running;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    // State setup
    app.init_state::<AppState>();
    app.enable_state_scoped_entities::<AppState>();
    app.add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>);

    // Setup(s), update(s), teardown(s)
    app.add_systems(Startup, setup);

    // Sub plugins
    app.add_plugins((loading::plugin, running::plugin));
}

/// Core state of the application.
#[derive(States, Debug, PartialEq, Hash, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    Loading,
    Running,
}
