use bevy::prelude::*;

use super::{BUTTON_HOVER, BUTTON_NORMAL, BUTTON_PRESSED};
pub struct MouseHoverPlugin;

impl Plugin for MouseHoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (mut background, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background = BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *background = BUTTON_HOVER.into();
            }
            Interaction::None => {
                *background = BUTTON_NORMAL.into();
            }
        }
    }
}
