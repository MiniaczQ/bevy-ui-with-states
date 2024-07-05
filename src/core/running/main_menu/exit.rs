use bevy::prelude::*;
use widgets::MyWidgets;

use crate::ui::*;

use super::MenuState;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Exit), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Exit)));
}

fn setup(mut commands: Commands) {
    let list = commands.ui_root().insert(StateScoped(MenuState::Exit)).id();

    commands.ui_label("Exit?").set_parent(list);

    commands
        .ui_button("Yes")
        .insert(UiAction::Yes)
        .set_parent(list);
    commands
        .ui_button("No")
        .insert(UiAction::No)
        .set_parent(list);
}

fn update(
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in &mut interaction_query {
        if interaction.just_released() {
            match button {
                UiAction::Yes => {
                    exit.send(AppExit::Success);
                }
                UiAction::No => next_menu_state.set(MenuState::Title),
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Yes,
    No,
}
