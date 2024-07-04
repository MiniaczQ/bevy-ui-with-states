use bevy::prelude::*;

use super::{GameAssets, GameState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(GameState::Loading)));
    }
}

fn update(
    mut next_state: ResMut<NextState<GameState>>,
    assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
) {
    if images.contains(&assets.player_image) {
        next_state.set(GameState::Playing);
    }
}
