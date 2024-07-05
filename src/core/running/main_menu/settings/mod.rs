mod audio;
mod controls;
mod graphics;

use bevy::prelude::*;
use widgets::MyWidgets;

use crate::ui::*;

use super::MenuState;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<SettingsState>();
    app.enable_state_scoped_entities::<SettingsState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<SettingsState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Settings), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Settings)));
    app.add_systems(OnExit(MenuState::Settings), teardown);

    // Sub plugins
    app.add_plugins((audio::plugin, controls::plugin, graphics::plugin));
}

fn setup(mut commands: Commands) {
    let list = commands
        .ui_root()
        .insert(StateScoped(MenuState::Settings))
        .id();

    let navbar = commands.ui_horizontal().set_parent(list).id();

    commands
        .ui_button("Graphics")
        .insert((UiAction::Graphics, graphics::Marker))
        .set_parent(navbar);
    commands
        .ui_button("Controls")
        .insert((UiAction::Controls, controls::Marker))
        .set_parent(navbar);
    commands
        .ui_button("Audio")
        .insert((UiAction::Audio, audio::Marker))
        .set_parent(navbar);

    let settings = commands.ui_vertical().set_parent(list).id();
    commands.insert_resource(SettingsList(settings));

    commands
        .ui_button("Back")
        .insert(UiAction::Back)
        .set_parent(list);
}

fn update(
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interaction_query {
        if interaction.just_released() {
            match button {
                UiAction::Back => next_menu_state.set(MenuState::Title),
                UiAction::Graphics => next_settings_state.set(SettingsState::Graphics),
                UiAction::Controls => next_settings_state.set(SettingsState::Controls),
                UiAction::Audio => next_settings_state.set(SettingsState::Audio),
            }
        }
    }
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<SettingsList>();
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(MenuState = MenuState::Settings)]
enum SettingsState {
    #[default]
    Graphics,
    Controls,
    Audio,
}

#[derive(Resource)]
struct SettingsList(Entity);

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Back,
    Graphics,
    Controls,
    Audio,
}
