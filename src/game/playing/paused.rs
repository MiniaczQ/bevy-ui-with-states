use bevy::prelude::*;

use crate::{ui::*, CoreState};

use super::{GameState, PauseState};

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        // Setup, update, teardown
        app.add_systems(OnEnter(PauseState::Paused), setup);
        app.add_systems(Update, update.run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands) {
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

fn update(
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

#[derive(Component, PartialEq, Eq)]
enum PauseButton {
    Unpause,
    MainMenu,
}
