mod core;
mod ui;
mod utility;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(core::plugin)
        .run();
}
