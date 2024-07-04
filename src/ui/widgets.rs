use bevy::{ecs::system::EntityCommands, prelude::*};

use super::MouseHover;

/// Helper trait for creating common widgets.
pub trait CommandsExtWidgets<'w> {
    fn my_root(&mut self) -> EntityCommands;
    fn my_vertical(&mut self) -> EntityCommands;
    fn my_horizontal(&mut self) -> EntityCommands;
    fn my_button<I: Into<String>, C: Component>(&mut self, text: I, comp: C) -> EntityCommands;
    fn my_label<I: Into<String>>(&mut self, text: I) -> EntityCommands;
}

impl<'w, 's> CommandsExtWidgets<'w> for Commands<'w, 's> {
    fn my_root(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
    }

    fn my_vertical(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
    }

    fn my_horizontal(&mut self) -> EntityCommands {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                row_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
    }

    fn my_button<I: Into<String>, C: Component>(&mut self, text: I, comp: C) -> EntityCommands {
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
                    ..default()
                },
                MouseHover,
                comp,
            ))
            .id();
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            },
        ))
        .set_parent(button);
        self.entity(button)
    }

    fn my_label<I: Into<String>>(&mut self, text: I) -> EntityCommands {
        let label = self
            .spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .id();
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.3, 0.3, 0.7),
                ..default()
            },
        ))
        .set_parent(label);
        self.entity(label)
    }
}
