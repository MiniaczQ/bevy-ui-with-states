use bevy::{ecs::system::EntityCommands, prelude::*};

use super::{BUTTON_ANIMATED_BACKGROUND, BUTTON_TEXT, LABEL_TEXT, NODE_DEFAULT};

/// Helper trait for common widgets.
pub trait MyWidgets<'w> {
    fn ui_root(&mut self) -> EntityCommands;
    fn ui_vertical(&mut self) -> EntityCommands;
    fn ui_horizontal(&mut self) -> EntityCommands;
    fn ui_button<I: Into<String>>(&mut self, text: I) -> EntityCommands;
    fn ui_label<I: Into<String>>(&mut self, text: I) -> EntityCommands;
}

impl<'w, 's> MyWidgets<'w> for Commands<'w, 's> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
    }

    fn ui_vertical(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        })
    }

    fn ui_horizontal(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        })
    }

    fn ui_button<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let button = self
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(NODE_DEFAULT),
                    ..default()
                },
                BUTTON_ANIMATED_BACKGROUND,
            ))
            .id();
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: BUTTON_TEXT,
                ..default()
            },
        ))
        .set_parent(button);
        self.entity(button)
    }

    fn ui_label<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let label = self
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(300.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_DEFAULT),
                ..default()
            })
            .id();
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: LABEL_TEXT,
                ..default()
            },
        ))
        .set_parent(label);
        self.entity(label)
    }
}
