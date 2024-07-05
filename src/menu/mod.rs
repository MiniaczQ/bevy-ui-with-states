mod credits;
mod exit;
mod settings;

use bevy::prelude::*;
use widgets::MyWidgets;

use crate::{core::CoreState, ui::*};

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<MenuState>();
    app.enable_state_scoped_entities::<MenuState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<MenuState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Main), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Main)));

    // Sub plugins
    app.add_plugins((settings::plugin, credits::plugin, exit::plugin));
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(CoreState = CoreState::Menu)]
pub enum MenuState {
    #[default]
    Main,
    Exit,
    Settings,
    Credits,
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
    mut core_state: ResMut<NextState<CoreState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interaction_query {
        if interaction.just_released() {
            match button {
                UiAction::Play => core_state.set(CoreState::Game),
                UiAction::Settings => menu_state.set(MenuState::Settings),
                UiAction::Credits => menu_state.set(MenuState::Credits),
                UiAction::Exit => menu_state.set(MenuState::Exit),
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
