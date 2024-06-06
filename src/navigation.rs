use crate::ui;
use crate::{states::*, ui::animation::UiAnimationPlugin};
use bevy::prelude::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiAnimationPlugin);

        app.add_systems(OnEnter(MenuState::Main), ui::main_setup)
            .add_systems(Update, ui::main_update.run_if(in_state(MenuState::Main)));

        app.add_systems(OnEnter(GameState::Loading), ui::loading_setup)
            .add_systems(
                Update,
                ui::loading_update.run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnExit(GameState::Loading), ui::loading_teardown);

        app.add_systems(OnEnter(PauseState::Paused), ui::pause_setup)
            .add_systems(
                Update,
                ui::pause_update.run_if(in_state(GameState::Playing)),
            );

        app.add_systems(OnEnter(MenuState::Exit), ui::exit_setup)
            .add_systems(Update, ui::exit_update.run_if(in_state(MenuState::Exit)));

        app.add_systems(OnEnter(MenuState::Credits), ui::credits_setup)
            .add_systems(
                Update,
                ui::credits_update.run_if(in_state(MenuState::Credits)),
            );

        app.add_systems(OnEnter(MenuState::Settings), ui::settings_setup)
            .add_systems(
                Update,
                ui::settings_update.run_if(in_state(MenuState::Settings)),
            )
            .add_systems(OnExit(MenuState::Settings), ui::settings_teardown);

        app.add_systems(OnEnter(SettingsState::Audio), ui::audio_setup)
            .add_systems(OnEnter(SettingsState::Controls), ui::controls_setup)
            .add_systems(OnEnter(SettingsState::Graphics), ui::graphics_setup);
    }
}
