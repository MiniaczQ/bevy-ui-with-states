//! Collection of UI helpers.
#![allow(dead_code)]

pub mod extended_interaction;
pub mod mouse_hover;
pub mod widgets;

use bevy::{color::palettes::tailwind, prelude::*};

use extended_interaction::ExtendedInteration;
use mouse_hover::AnimatedBackground;

pub const NODE_DEFAULT: Color = Color::Srgba(tailwind::GRAY_800);
pub const BACKGROUND: Color = Color::Srgba(tailwind::GRAY_900);

pub const BUTTON_HOVERED: Color = Color::Srgba(tailwind::GRAY_600);
pub const BUTTON_PRESSED: Color = Color::Srgba(tailwind::GRAY_400);
pub const BUTTON_TEXT: Color = Color::Srgba(tailwind::GRAY_100);
pub const BUTTON_ANIMATED_BACKGROUND: AnimatedBackground =
    AnimatedBackground::all(NODE_DEFAULT, BUTTON_HOVERED, BUTTON_PRESSED);
pub const FROZEN_BUTTON_ANIMATED_BACKGROUND: AnimatedBackground =
    AnimatedBackground::none(BUTTON_PRESSED);

pub const LABEL_HOVERED: Color = Color::Srgba(tailwind::AMBER_600);
pub const LABEL_TEXT: Color = Color::Srgba(tailwind::AMBER_300);
pub const LABEL_ANIMATED_BACKGROUND: AnimatedBackground =
    AnimatedBackground::hovered(NODE_DEFAULT, LABEL_HOVERED);

pub type ButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (&'a ExtendedInteration, &'a T), (Changed<ExtendedInteration>, With<Button>)>;

pub(super) fn plugin(app: &mut App) {
    // Sub plugins
    app.add_plugins((mouse_hover::plugin, extended_interaction::plugin));

    // Other
    app.insert_resource(ClearColor(BACKGROUND));
}
