mod game;
mod menu;
mod ui;

use bevy::{dev_tools::states::log_transitions, prelude::*};

use game::GamePlugin;
use menu::MenuPlugin;
use ui::MouseHoverPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MouseHoverPlugin, CorePlugin))
        .run();
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Setup state
        app.init_state::<CoreState>()
            .enable_state_scoped_entities::<CoreState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<CoreState>);

        // Setup, update, teardown
        app.add_systems(Startup, setup);

        // Sub plugins
        app.add_plugins((MenuPlugin, GamePlugin));
    }
}

#[derive(States, Debug, PartialEq, Hash, Eq, Clone, Default)]
pub enum CoreState {
    #[default]
    Menu,
    Game,
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
