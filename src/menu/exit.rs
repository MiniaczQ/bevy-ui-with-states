use bevy::prelude::*;

use crate::ui::*;

use super::MenuState;

pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        // Setup, update, teardown
        app.add_systems(OnEnter(MenuState::Exit), setup);
        app.add_systems(Update, update.run_if(in_state(MenuState::Exit)));
    }
}

fn setup(mut commands: Commands) {
    let list = commands.my_root().insert(StateScoped(MenuState::Exit)).id();

    commands.my_label("Exit?").set_parent(list);

    commands.my_button("Yes", UiAction::Yes).set_parent(list);
    commands.my_button("No", UiAction::No).set_parent(list);
}

fn update(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            match button {
                UiAction::Yes => {
                    exit.send(AppExit::Success);
                }
                UiAction::No => menu_state.set(MenuState::Main),
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Yes,
    No,
}
