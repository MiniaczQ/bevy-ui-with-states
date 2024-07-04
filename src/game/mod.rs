mod loading;
mod playing;

use bevy::{dev_tools::states::log_transitions, prelude::*};
use loading::LoadingPlugin;
use playing::PlayingPlugin;

use crate::CoreState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup state
        app.add_sub_state::<GameState>()
            .enable_state_scoped_entities::<GameState>();
        #[cfg(debug_assertions)]
        app.add_systems(Update, log_transitions::<GameState>);

        // Setup, update, teardown
        app.add_systems(OnEnter(CoreState::Game), setup);

        // Sub plugins
        app.add_plugins((LoadingPlugin, PlayingPlugin));
    }
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(CoreState = CoreState::Game)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Resource)]
pub struct GameAssets {
    pub player_image: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_image: asset_server.load("square.png"),
    });
}
