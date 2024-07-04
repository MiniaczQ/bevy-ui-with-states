use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::ui::*;

use super::MenuState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Setup state
        app.add_sub_state::<SettingsState>()
            .enable_state_scoped_entities::<SettingsState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<SettingsState>);

        // Setup, update, teardown
        app.add_systems(OnEnter(MenuState::Settings), setup);
        app.add_systems(Update, update.run_if(in_state(MenuState::Settings)));
        app.add_systems(OnExit(MenuState::Settings), teardown);

        // Sub plugins
        app.add_systems(OnEnter(SettingsState::Audio), setup_audio);
        app.add_systems(OnEnter(SettingsState::Controls), setup_controls);
        app.add_systems(OnEnter(SettingsState::Graphics), setup_graphics);
    }
}

fn setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(MenuState::Settings))
        .id();

    let navbar = commands.my_horizontal().set_parent(list).id();

    commands
        .my_button("Graphics", UiAction::Graphics)
        .set_parent(navbar);
    commands
        .my_button("Controls", UiAction::Controls)
        .set_parent(navbar);
    commands
        .my_button("Audio", UiAction::Audio)
        .set_parent(navbar);

    let settings = commands.my_vertical().set_parent(list).id();
    commands.insert_resource(SettingsList(settings));

    commands.my_button("Back", UiAction::Back).set_parent(list);
}

fn update(
    mut menu_next: ResMut<NextState<MenuState>>,
    mut settings_next: ResMut<NextState<SettingsState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                UiAction::Back => menu_next.set(MenuState::Main),
                UiAction::Graphics => settings_next.set(SettingsState::Graphics),
                UiAction::Controls => settings_next.set(SettingsState::Controls),
                UiAction::Audio => settings_next.set(SettingsState::Audio),
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

fn setup_graphics(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .my_vertical()
        .insert(StateScoped(SettingsState::Graphics))
        .set_parent(root.0)
        .id();

    commands.my_label("Brightness").set_parent(list);
    commands.my_label("Details").set_parent(list);
}

fn setup_controls(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .my_vertical()
        .insert(StateScoped(SettingsState::Controls))
        .set_parent(root.0)
        .id();

    commands.my_label("Forward").set_parent(list);
    commands.my_label("Backward").set_parent(list);
    commands.my_label("Attack").set_parent(list);
}

fn setup_audio(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .my_vertical()
        .insert(StateScoped(SettingsState::Audio))
        .set_parent(root.0)
        .id();

    commands.my_label("Volume").set_parent(list);
}
