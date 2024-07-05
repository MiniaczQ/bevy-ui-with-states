use bevy::prelude::*;
use widgets::MyWidgets;

use crate::ui::*;

use super::MenuState;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Credits), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Credits)));
}

pub fn setup(mut commands: Commands) {
    let root = commands
        .ui_root()
        .insert(StateScoped(MenuState::Credits))
        .id();
    let table = commands.ui_horizontal().set_parent(root).id();
    let credit_list = commands.ui_vertical().set_parent(table).id();
    let author_list = commands.ui_vertical().set_parent(table).id();

    commands.ui_label("Music:").set_parent(credit_list);
    commands.ui_label("Art:").set_parent(credit_list);

    commands.ui_label("Alice").set_parent(author_list);
    commands.ui_label("Bob").set_parent(author_list);

    commands.ui_button("Back").insert(UiAction).set_parent(root);
}

pub fn update(
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, _) in &mut interaction_query {
        if interaction.just_released() {
            next_menu_state.set(MenuState::Main)
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct UiAction;
