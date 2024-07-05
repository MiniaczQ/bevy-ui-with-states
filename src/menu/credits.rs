use bevy::prelude::*;

use crate::ui::*;

use super::MenuState;

pub(super) fn plugin(app: &mut App) {
    // Setup, update, teardown
    app.add_systems(OnEnter(MenuState::Credits), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Credits)));
}

pub fn setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(MenuState::Credits))
        .id();

    commands.my_label("Alice").set_parent(list);
    commands.my_label("Bob").set_parent(list);

    commands.my_button("Back").insert(UiAction).set_parent(list);
}

pub fn update(
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, _) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            menu_state.set(MenuState::Main)
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct UiAction;
