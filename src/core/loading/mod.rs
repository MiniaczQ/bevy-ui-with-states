use std::time::Duration;

use bevy::prelude::*;

use crate::{ui::widgets::MyWidgets, utility::barrier::*};

use super::AppState;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(AppState::Loading), (add_barrier, setup_ui));
    app.add_systems(
        Update,
        ((minimum_load_time), done)
            .chain()
            .run_if(in_state(AppState::Loading)),
    );
}

#[derive(Component)]
struct AppLoading;

fn add_barrier(mut commands: Commands) {
    commands.spawn((
        Barrier::default(),
        AppLoading,
        StateScoped(AppState::Loading),
    ));
}

fn setup_ui(mut commands: Commands) {
    let root = commands
        .ui_root()
        .insert(StateScoped(AppState::Loading))
        .id();
    commands.ui_label("Loading").set_parent(root);
}

fn minimum_load_time(
    mut barrier: BarrierBlocker<AppLoading>,
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

fn done(mut next_app_state: ResMut<NextState<AppState>>, barrier: BarrierAwaiter<AppLoading>) {
    if barrier.is_completed() {
        next_app_state.set(AppState::Running);
    }
}
