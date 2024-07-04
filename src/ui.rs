//! Collection of UI helpers.

use bevy::{ecs::system::EntityCommands, prelude::*};

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

pub struct MouseHoverPlugin;

impl Plugin for MouseHoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_hover_update);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MouseHover;

fn mouse_hover_update(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<MouseHover>),
    >,
) {
    for (interaction, mut image) in &mut interaction_query {
        let color = &mut image.color;
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON;
            }
        }
    }
}

pub type ButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;
