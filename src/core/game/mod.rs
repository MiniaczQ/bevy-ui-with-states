mod loading;
mod playing;

use bevy::prelude::*;

use crate::core::CoreState;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<GameState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(CoreState::Game), setup);
    app.add_systems(OnExit(CoreState::Game), teardown);

    // Sub plugins
    app.add_plugins((loading::plugin, playing::plugin));
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(CoreState = CoreState::Game)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Resource)]
struct GameAssets {
    pub player_image: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_image: asset_server.load("bevy.png"),
    });
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<GameAssets>();
}
