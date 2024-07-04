use bevy::{dev_tools::states::log_transitions, prelude::*};
use paused::PausedPlugin;
use unpaused::UnpausedPlugin;
mod paused;
mod unpaused;

use super::GameState;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        // Setup state
        app.add_sub_state::<PauseState>()
            .enable_state_scoped_entities::<PauseState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<PauseState>);

        // Setup, update, teardown
        app.add_systems(OnEnter(GameState::Playing), setup);

        // Sub plugins
        app.add_plugins((UnpausedPlugin, PausedPlugin));
    }
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(GameState = GameState::Playing)]
enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        StateScoped(GameState::Playing),
        SpriteBundle {
            texture: asset_server.load("square.png"),
            ..default()
        },
    ));
}
