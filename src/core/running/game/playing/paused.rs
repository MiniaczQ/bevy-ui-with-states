use bevy::prelude::*;
use widgets::MyWidgets;

use crate::{
    core::{running::RunningState, AppState},
    ui::*,
};

use super::PauseState;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(PauseState::Paused), setup);
    app.add_systems(Update, update.run_if(in_state(PauseState::Paused)));
}

fn setup(mut commands: Commands) {
    let list = commands
        .ui_root()
        .insert(StateScoped(PauseState::Paused))
        .id();

    commands.ui_label("Paused").set_parent(list);
    commands
        .ui_button("Unpause")
        .insert(UiAction::Unpause)
        .set_parent(list);
    commands
        .ui_button("Main Menu")
        .insert(UiAction::MainMenu)
        .set_parent(list);
}

fn update(
    mut interaction_query: ButtonQuery<UiAction>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_running_state: ResMut<NextState<RunningState>>,
) {
    for (interaction, button) in &mut interaction_query {
        if interaction.just_released() {
            match button {
                UiAction::Unpause => next_pause_state.set(PauseState::Unpaused),
                UiAction::MainMenu => next_running_state.set(RunningState::Menu),
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Unpause,
    MainMenu,
}
