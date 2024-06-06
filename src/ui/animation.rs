use bevy::prelude::*;

pub struct UiAnimationPlugin;

impl Plugin for UiAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_animation);
    }
}

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button_animation(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<Button>),
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
