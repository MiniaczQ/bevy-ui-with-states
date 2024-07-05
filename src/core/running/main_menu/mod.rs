mod credits;
mod exit;
mod settings;
mod title;

use bevy::prelude::*;

use super::RunningState;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<MenuState>();
    app.enable_state_scoped_entities::<MenuState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<MenuState>,
    );

    // Sub plugins
    app.add_plugins((
        title::plugin,
        settings::plugin,
        credits::plugin,
        exit::plugin,
    ));
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(RunningState = RunningState::Menu)]
pub enum MenuState {
    #[default]
    Title,
    Exit,
    Settings,
    Credits,
}
