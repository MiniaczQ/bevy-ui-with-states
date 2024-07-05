use bevy::prelude::*;

use crate::{core::CoreState, ui::*};

use super::PauseState;

pub(super) fn plugin(app: &mut App) {
    // Setup, update, teardown
    app.add_systems(OnEnter(PauseState::Paused), setup);
    app.add_systems(Update, update.run_if(in_state(PauseState::Paused)));
}

fn setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(PauseState::Paused))
        .id();

    commands.my_label("Paused").set_parent(list);
    commands
        .my_button("Unpause")
        .insert(UiAction::Unpause)
        .set_parent(list);
    commands
        .my_button("Main Menu")
        .insert(UiAction::MainMenu)
        .set_parent(list);
}

fn update(
    mut interaction_query: ButtonQuery<UiAction>,
    mut pause_next: ResMut<NextState<PauseState>>,
    mut core_next: ResMut<NextState<CoreState>>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                UiAction::Unpause => pause_next.set(PauseState::Unpaused),
                UiAction::MainMenu => core_next.set(CoreState::Menu),
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Unpause,
    MainMenu,
}
