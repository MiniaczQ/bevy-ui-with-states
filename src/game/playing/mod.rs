use bevy::prelude::*;
mod paused;
mod unpaused;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    // Setup state
    app.add_sub_state::<PauseState>();
    app.enable_state_scoped_entities::<PauseState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<PauseState>,
    );

    // Setup, update, teardown
    app.add_systems(OnEnter(GameState::Playing), setup);

    // Sub plugins
    app.add_plugins((unpaused::plugin, paused::plugin));
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
