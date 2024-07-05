use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate_background);
}

/// Component for animating background based on ongoing interaction.
#[derive(Component)]
pub struct AnimatedBackground {
    none: Color,
    hovered: Option<Color>,
    pressed: Option<Color>,
}

impl AnimatedBackground {
    pub const fn all(none: Color, hovered: Color, pressed: Color) -> Self {
        Self {
            none,
            hovered: Some(hovered),
            pressed: Some(pressed),
        }
    }

    pub const fn hovered(none: Color, hovered: Color) -> Self {
        Self {
            none,
            hovered: Some(hovered),
            pressed: None,
        }
    }

    pub const fn none(none: Color) -> Self {
        Self {
            none,
            hovered: None,
            pressed: None,
        }
    }
}

fn animate_background(
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction, &AnimatedBackground),
        Or<(Changed<Interaction>, Changed<AnimatedBackground>)>,
    >,
) {
    for (mut background, interaction, animated) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *background = animated.none.into();
            }
            Interaction::Hovered => {
                if let Some(hovered) = animated.hovered {
                    *background = hovered.into();
                }
            }
            Interaction::Pressed => {
                if let Some(pressed) = animated.pressed {
                    *background = pressed.into();
                }
            }
        }
    }
}
