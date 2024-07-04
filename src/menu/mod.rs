use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::{ui::*, CoreState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<MenuState>);

        app.add_systems(OnEnter(MenuState::Main), setup)
            .add_systems(Update, update.run_if(in_state(MenuState::Main)));
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
pub enum UiAction {
    Play,
    Settings,
    Credits,
    Exit,
}

pub fn exit_setup(mut commands: Commands) {
    let list = commands.my_root().insert(StateScoped(MenuState::Exit)).id();

    commands.my_label("Exit?").set_parent(list);

    commands.my_button("Yes", ExitButton::Yes).set_parent(list);
    commands.my_button("No", ExitButton::No).set_parent(list);
}

#[derive(Component, PartialEq, Eq)]
pub enum ExitButton {
    Yes,
    No,
}

pub fn exit_update(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: ButtonQuery<ExitButton>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                ExitButton::Yes => {
                    exit.send(AppExit::Success);
                }
                ExitButton::No => menu_state.set(MenuState::Main),
            }
        }
    }
}

pub fn credits_setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(MenuState::Credits))
        .id();

    commands.my_label("Alice").set_parent(list);
    commands.my_label("Bob").set_parent(list);

    commands.my_button("Back", CreditsButton).set_parent(list);
}

#[derive(Component, PartialEq, Eq)]
pub struct CreditsButton;

pub fn credits_update(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<CreditsButton>,
) {
    for (interaction, _) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            menu_state.set(MenuState::Main)
        }
    }
}

pub fn settings_setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(MenuState::Settings))
        .id();

    let navbar = commands.my_horizontal().set_parent(list).id();

    commands
        .my_button("Graphics", SettingsButton::Graphics)
        .set_parent(navbar);
    commands
        .my_button("Controls", SettingsButton::Controls)
        .set_parent(navbar);
    commands
        .my_button("Audio", SettingsButton::Audio)
        .set_parent(navbar);

    let settings = commands.my_vertical().set_parent(list).id();
    commands.insert_resource(SettingsList(settings));

    commands
        .my_button("Back", SettingsButton::Back)
        .set_parent(list);
}

#[derive(Component, PartialEq, Eq)]
pub enum SettingsButton {
    Back,
    Graphics,
    Controls,
    Audio,
}

#[derive(Resource)]
pub struct SettingsList(Entity);

pub fn settings_update(
    mut menu_next: ResMut<NextState<MenuState>>,
    mut settings_next: ResMut<NextState<SettingsState>>,
    mut interaction_query: ButtonQuery<SettingsButton>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                SettingsButton::Back => menu_next.set(MenuState::Main),
                SettingsButton::Graphics => settings_next.set(SettingsState::Graphics),
                SettingsButton::Controls => settings_next.set(SettingsState::Controls),
                SettingsButton::Audio => settings_next.set(SettingsState::Audio),
            }
        }
    }
}

pub fn settings_teardown(mut commands: Commands) {
    commands.remove_resource::<SettingsList>();
}

pub fn graphics_setup(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .my_vertical()
        .insert(StateScoped(SettingsState::Graphics))
        .set_parent(root.0)
        .id();

    commands.my_label("Brightness").set_parent(list);
    commands.my_label("Details").set_parent(list);
}

pub fn controls_setup(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .my_vertical()
        .insert(StateScoped(SettingsState::Controls))
        .set_parent(root.0)
        .id();

    commands.my_label("Forward").set_parent(list);
    commands.my_label("Backward").set_parent(list);
    commands.my_label("Attack").set_parent(list);
}

pub fn audio_setup(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .my_vertical()
        .insert(StateScoped(SettingsState::Audio))
        .set_parent(root.0)
        .id();

    commands.my_label("Volume").set_parent(list);
}

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Credits), credits_setup)
            .add_systems(Update, credits_update.run_if(in_state(MenuState::Credits)));
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Settings), settings_setup)
            .add_systems(
                Update,
                settings_update.run_if(in_state(MenuState::Settings)),
            )
            .add_systems(OnExit(MenuState::Settings), settings_teardown);

        app.add_systems(OnEnter(SettingsState::Audio), audio_setup)
            .add_systems(OnEnter(SettingsState::Controls), controls_setup)
            .add_systems(OnEnter(SettingsState::Graphics), graphics_setup);
    }
}

pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Exit), exit_setup)
            .add_systems(Update, exit_update.run_if(in_state(MenuState::Exit)));
    }
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SettingsState>()
            .enable_state_scoped_entities::<SettingsState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<SettingsState>);
    }
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(MenuState = MenuState::Settings)]
pub enum SettingsState {
    #[default]
    Graphics,
    Controls,
    Audio,
}
