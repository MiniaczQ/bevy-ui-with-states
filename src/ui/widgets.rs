use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait CommandsExtWidgets<'w> {
    fn ui_root_list(&mut self) -> EntityCommands;
    fn ui_vlist(&mut self) -> EntityCommands;
    fn ui_hlist(&mut self) -> EntityCommands;
    fn ui_button<I: Into<String>, C: Component>(&mut self, text: I, comp: C) -> EntityCommands;
    fn ui_label<I: Into<String>>(&mut self, text: I) -> EntityCommands;
}

impl<'w, 's> CommandsExtWidgets<'w> for Commands<'w, 's> {
    fn ui_root_list(&mut self) -> EntityCommands {
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

    fn ui_vlist(&mut self) -> EntityCommands {
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

    fn ui_hlist(&mut self) -> EntityCommands {
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

    fn ui_button<I: Into<String>, C: Component>(&mut self, text: I, comp: C) -> EntityCommands {
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

    fn ui_label<I: Into<String>>(&mut self, text: I) -> EntityCommands {
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
