use std::time::Duration;

use bevy::prelude::*;

use crate::{ui::widgets::MyWidgets, utility::barrier::*};

use super::{GameAssets, GameState};

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(GameState::Loading), (add_barrier, setup_ui));
    app.add_systems(
        Update,
        ((wait_for_assets, minimum_load_time), done)
            .chain()
            .run_if(in_state(GameState::Loading)),
    );
}

#[derive(Component)]
struct GameLoading;

fn add_barrier(mut commands: Commands) {
    commands.spawn((
        Barrier::default(),
        GameLoading,
        StateScoped(GameState::Loading),
    ));
}

fn setup_ui(mut commands: Commands) {
    let root = commands
        .ui_root()
        .insert(StateScoped(GameState::Loading))
        .id();
    commands.ui_label("Loading").set_parent(root);
}

fn wait_for_assets(
    mut barrier: BarrierBlocker<GameLoading>,
    assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
) {
    if barrier.is_completed() {
        return;
    }
    if !barrier.is_registered() {
        barrier.register();
        return;
    }
    if images.contains(&assets.player_image) {
        barrier.complete();
    }
}

fn minimum_load_time(
    mut barrier: BarrierBlocker<GameLoading>,
    mut started: Local<Option<Duration>>,
    time: Res<Time>,
) {
    if barrier.is_completed() {
        return;
    }
    if !barrier.is_registered() {
        *started = Some(time.elapsed());
        barrier.register();
        return;
    }
    let started = started.unwrap();
    if started + Duration::from_secs(2) < time.elapsed() {
        barrier.complete();
    }
}

fn done(mut next_game_state: ResMut<NextState<GameState>>, barrier: BarrierAwaiter<GameLoading>) {
    if barrier.is_completed() {
        next_game_state.set(GameState::Playing);
    }
}
