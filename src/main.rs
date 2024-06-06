mod game;
mod navigation;
mod states;
mod ui;

use bevy::prelude::*;

use game::GamePlugin;
use navigation::NavigationPlugin;
use states::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StatesPlugin, NavigationPlugin, GamePlugin))
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
