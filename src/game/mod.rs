use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::{ui::*, CoreState};

pub fn pause_update(
    input: Res<ButtonInput<KeyCode>>,
    pause: Res<State<PauseState>>,
    mut interaction_query: ButtonQuery<PauseButton>,
    mut pause_next: ResMut<NextState<PauseState>>,
    mut core_next: ResMut<NextState<CoreState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match pause.get() {
            PauseState::Unpaused => pause_next.set(PauseState::Paused),
            PauseState::Paused => pause_next.set(PauseState::Unpaused),
        }
    }

    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                PauseButton::Unpause => pause_next.set(PauseState::Unpaused),
                PauseButton::MainMenu => core_next.set(CoreState::Menu),
            }
        }
    }
}

#[derive(Resource)]
pub struct LoadingCountdown(u32);

pub fn loading_setup(mut commands: Commands) {
    commands.insert_resource(LoadingCountdown(60));
}

pub fn loading_update(
    mut countdown: ResMut<LoadingCountdown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if countdown.0 > 0 {
        countdown.0 -= 1;
    }
    if countdown.0 == 0 {
        next_state.set(GameState::Playing);
    }
    info!("Loading, frames left: {}", countdown.0);
}

pub fn loading_teardown(mut commands: Commands) {
    commands.remove_resource::<LoadingCountdown>();
}

pub fn pause_setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(PauseState::Paused))
        .id();

    commands.my_label("Paused").set_parent(list);

    commands
        .my_button("Unpause", PauseButton::Unpause)
        .set_parent(list);
    commands
        .my_button("Main Menu", PauseButton::MainMenu)
        .set_parent(list);
}

#[derive(Component, PartialEq, Eq)]
pub enum PauseButton {
    Unpause,
    MainMenu,
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), loading_setup)
            .add_systems(Update, loading_update.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), loading_teardown);
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PauseState::Paused), pause_setup)
            .add_systems(Update, pause_update.run_if(in_state(GameState::Playing)));
    }
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GameState>()
            .enable_state_scoped_entities::<GameState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<GameState>);

        app.add_sub_state::<PauseState>()
            .enable_state_scoped_entities::<PauseState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<PauseState>);
    }
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(CoreState = CoreState::Game)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(GameState = GameState::Playing)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(
                Update,
                playing_update.run_if(in_state(PauseState::Unpaused)),
            );
    }
}

pub fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        StateScoped(GameState::Playing),
        SpriteBundle {
            texture: asset_server.load("square.png"),
            ..default()
        },
    ));
}

pub fn playing_update(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * time.delta_seconds() * 300.0;
        }
    }
}
