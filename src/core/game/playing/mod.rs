use bevy::prelude::*;
mod paused;
mod unpaused;

use super::{GameAssets, GameState};

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<PauseState>();
    app.enable_state_scoped_entities::<PauseState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<PauseState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(GameState::Playing), setup);
    app.add_systems(Update, update.run_if(in_state(GameState::Playing)));

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

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        StateScoped(GameState::Playing),
        SpriteBundle {
            texture: assets.player_image.clone(),
            ..default()
        },
    ));
}

fn update(
    input: Res<ButtonInput<KeyCode>>,
    pause: Res<State<PauseState>>,
    mut pause_next: ResMut<NextState<PauseState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match pause.get() {
            PauseState::Unpaused => pause_next.set(PauseState::Paused),
            PauseState::Paused => pause_next.set(PauseState::Unpaused),
        }
    }
}
