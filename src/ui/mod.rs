//! Collection of UI helpers.

mod mouse_hover;
mod widgets;

use bevy::prelude::*;

pub use mouse_hover::*;
pub use widgets::*;

pub type ButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;
