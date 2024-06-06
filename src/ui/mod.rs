use bevy::prelude::*;

use crate::states::*;

pub mod animation;
mod widgets;

use widgets::*;

type ButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;

pub fn main_setup(mut commands: Commands) {
    let list = commands
        .ui_root_list()
        .insert(StateScoped(MenuState::Main))
        .id();

    commands
        .ui_button("Play", MainButton::Play)
        .set_parent(list);
    commands
        .ui_button("Settings", MainButton::Settings)
        .set_parent(list);
    commands
        .ui_button("Credits", MainButton::Credits)
        .set_parent(list);

    #[cfg(not(target = "wasm"))]
    commands
        .ui_button("Exit", MainButton::Exit)
        .set_parent(list);
}

pub fn main_update(
    mut core_state: ResMut<NextState<CoreState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<MainButton>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                MainButton::Play => core_state.set(CoreState::Game),
                MainButton::Settings => menu_state.set(MenuState::Settings),
                MainButton::Credits => menu_state.set(MenuState::Credits),
                MainButton::Exit => menu_state.set(MenuState::Exit),
            }
        }
    }
}

pub fn pause_update(
    input: Res<ButtonInput<KeyCode>>,
    pause: Res<State<PauseState>>,
    mut interaction_query: ButtonQuery<PauseButton>,
    mut pause_next: ResMut<NextState<PauseState>>,
    mut core_next: ResMut<NextState<CoreState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match pause.get() {
            PauseState::Unpaused => pause_next.set(PauseState::Paused),
            PauseState::Paused => pause_next.set(PauseState::Unpaused),
        }
    }

    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                PauseButton::Unpause => pause_next.set(PauseState::Unpaused),
                PauseButton::MainMenu => core_next.set(CoreState::Menu),
            }
        }
    }
}

#[derive(Resource)]
pub struct LoadingCountdown(u32);

pub fn loading_setup(mut commands: Commands) {
    commands.insert_resource(LoadingCountdown(60));
}

pub fn loading_update(
    mut countdown: ResMut<LoadingCountdown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if countdown.0 > 0 {
        countdown.0 -= 1;
    } else {
        next_state.set(GameState::Playing);
    }
    info!("Loading: {}", countdown.0);
}

pub fn loading_teardown(mut commands: Commands) {
    commands.remove_resource::<LoadingCountdown>();
}

#[derive(Component, PartialEq, Eq)]
pub enum MainButton {
    Play,
    Settings,
    Credits,
    Exit,
}

pub fn pause_setup(mut commands: Commands) {
    let list = commands
        .ui_root_list()
        .insert(StateScoped(PauseState::Paused))
        .id();

    commands.ui_label("Paused").set_parent(list);

    commands
        .ui_button("Unpause", PauseButton::Unpause)
        .set_parent(list);
    commands
        .ui_button("Main Menu", PauseButton::MainMenu)
        .set_parent(list);
}

#[derive(Component, PartialEq, Eq)]
pub enum PauseButton {
    Unpause,
    MainMenu,
}

pub fn exit_setup(mut commands: Commands) {
    let list = commands
        .ui_root_list()
        .insert(StateScoped(MenuState::Exit))
        .id();

    commands.ui_label("Exit?").set_parent(list);

    commands.ui_button("Yes", ExitButton::Yes).set_parent(list);
    commands.ui_button("No", ExitButton::No).set_parent(list);
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
        .ui_root_list()
        .insert(StateScoped(MenuState::Credits))
        .id();

    commands.ui_label("Alice").set_parent(list);
    commands.ui_label("Bob").set_parent(list);

    commands.ui_button("Back", CreditsButton).set_parent(list);
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
        .ui_root_list()
        .insert(StateScoped(MenuState::Settings))
        .id();

    let navbar = commands.ui_hlist().set_parent(list).id();

    commands
        .ui_button("Graphics", SettingsButton::Graphics)
        .set_parent(navbar);
    commands
        .ui_button("Controls", SettingsButton::Controls)
        .set_parent(navbar);
    commands
        .ui_button("Audio", SettingsButton::Audio)
        .set_parent(navbar);

    let settings = commands.ui_vlist().id();
    commands.ui_label("a").set_parent(settings);
    commands.insert_resource(SettingsList(settings));

    commands
        .ui_button("Back", SettingsButton::Back)
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
        .ui_vlist()
        .insert(StateScoped(SettingsState::Graphics))
        .set_parent(root.0)
        .id();

    commands.ui_label("Brightness").set_parent(list);
    commands.ui_label("Details").set_parent(list);
}

pub fn controls_setup(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .ui_vlist()
        .insert(StateScoped(SettingsState::Controls))
        .set_parent(root.0)
        .id();

    commands.ui_label("Forward").set_parent(list);
    commands.ui_label("Backward").set_parent(list);
    commands.ui_label("Attack").set_parent(list);
}

pub fn audio_setup(mut commands: Commands, root: Res<SettingsList>) {
    let list = commands
        .ui_vlist()
        .insert(StateScoped(SettingsState::Audio))
        .set_parent(root.0)
        .id();

    commands.ui_label("Volume").set_parent(list);
}
