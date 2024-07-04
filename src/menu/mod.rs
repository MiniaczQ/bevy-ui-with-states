mod credits;
mod exit;
mod settings;

use bevy::{dev_tools::states::log_transitions, prelude::*};
use credits::CreditsPlugin;
use exit::ExitPlugin;
use settings::SettingsPlugin;

use crate::{ui::*, CoreState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Setup state
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<MenuState>);

        // Setup, update, teardown
        app.add_systems(OnEnter(MenuState::Main), setup);
        app.add_systems(Update, update.run_if(in_state(MenuState::Main)));

        // Sub plugins
        app.add_plugins((SettingsPlugin, CreditsPlugin, ExitPlugin));
    }
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
    let list = commands.my_root().insert(StateScoped(MenuState::Main)).id();

    commands.my_button("Play", UiAction::Play).set_parent(list);
    commands
        .my_button("Settings", UiAction::Settings)
        .set_parent(list);
    commands
        .my_button("Credits", UiAction::Credits)
        .set_parent(list);

    #[cfg(not(target = "wasm"))]
    commands.my_button("Exit", UiAction::Exit).set_parent(list);
}

fn update(
    mut core_state: ResMut<NextState<CoreState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
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
