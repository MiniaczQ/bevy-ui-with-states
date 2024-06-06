use bevy::dev_tools::states::log_transitions;
pub use bevy::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CoreState>()
            .add_sub_state::<GameState>()
            .add_sub_state::<PauseState>()
            .add_sub_state::<MenuState>()
            .add_sub_state::<SettingsState>()
            .enable_state_scoped_entities::<CoreState>()
            .enable_state_scoped_entities::<GameState>()
            .enable_state_scoped_entities::<PauseState>()
            .enable_state_scoped_entities::<MenuState>()
            .enable_state_scoped_entities::<SettingsState>();
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            (
                log_transitions::<CoreState>,
                log_transitions::<GameState>,
                log_transitions::<PauseState>,
                log_transitions::<MenuState>,
                log_transitions::<SettingsState>,
            ),
        );
    }
}

#[derive(States, Debug, PartialEq, Hash, Eq, Clone, Default)]
pub enum CoreState {
    #[default]
    Menu,
    Game,
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

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(CoreState = CoreState::Menu)]
pub enum MenuState {
    #[default]
    Main,
    Exit,
    Settings,
    Credits,
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(MenuState = MenuState::Settings)]
pub enum SettingsState {
    #[default]
    Graphics,
    Controls,
    Audio,
}
