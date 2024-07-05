use bevy::prelude::*;
use widgets::MyWidgets;

use crate::{core::AppState, ui::*};

use super::{MenuState, RunningState};

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Main), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Main)));
}

fn setup(mut commands: Commands) {
    let list = commands.ui_root().insert(StateScoped(MenuState::Main)).id();

    commands
        .ui_button("Play")
        .insert(UiAction::Play)
        .set_parent(list);
    commands
        .ui_button("Settings")
        .insert(UiAction::Settings)
        .set_parent(list);
    commands
        .ui_button("Credits")
        .insert(UiAction::Credits)
        .set_parent(list);

    #[cfg(not(target = "wasm"))]
    commands
        .ui_button("Exit")
        .insert(UiAction::Exit)
        .set_parent(list);
}

fn update(
    mut next_running_state: ResMut<NextState<RunningState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interaction_query {
        if interaction.just_released() {
            match button {
                UiAction::Play => next_running_state.set(RunningState::Game),
                UiAction::Settings => next_menu_state.set(MenuState::Settings),
                UiAction::Credits => next_menu_state.set(MenuState::Credits),
                UiAction::Exit => next_menu_state.set(MenuState::Exit),
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Play,
    Settings,
    Credits,
    Exit,
}
