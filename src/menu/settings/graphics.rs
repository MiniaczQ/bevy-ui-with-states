use bevy::prelude::*;
use mouse_hover::AnimatedBackground;
use widgets::MyWidgets;

use crate::ui::*;

use super::{SettingsList, SettingsState};

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(SettingsState::Graphics), setup_graphics);
    app.add_systems(OnExit(SettingsState::Graphics), teardown_graphics);
}

fn setup_graphics(
    mut commands: Commands,
    mut query: Query<&mut AnimatedBackground, With<Marker>>,
    root: Res<SettingsList>,
) {
    let table = commands
        .ui_horizontal()
        .insert(StateScoped(SettingsState::Graphics))
        .set_parent(root.0)
        .id();
    let label_list = commands.ui_vertical().set_parent(table).id();
    let key_list = commands.ui_vertical().set_parent(table).id();

    commands.ui_label("Brightness").set_parent(label_list);
    commands.ui_label("Details").set_parent(label_list);

    commands.ui_button("Low").set_parent(key_list);
    commands.ui_button("Low").set_parent(key_list);

    *query.single_mut() = FROZEN_BUTTON_ANIMATED_BACKGROUND;
}

fn teardown_graphics(mut query: Query<&mut AnimatedBackground, With<Marker>>) {
    *query.single_mut() = BUTTON_ANIMATED_BACKGROUND;
}

#[derive(Component)]
pub(super) struct Marker;
