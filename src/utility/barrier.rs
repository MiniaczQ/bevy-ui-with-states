//! Synchronization primitive.

use bevy::{ecs::system::SystemParam, prelude::*};

/// Core data of a barrier.
#[derive(Component, Default)]
pub struct Barrier {
    remaining: usize,
}

impl Barrier {
    /// Registers a new blocker.
    fn register(&mut self) {
        self.remaining += 1;
    }

    /// Completes a blocker.
    fn complete(&mut self) {
        self.remaining = self.remaining.saturating_sub(1);
    }

    /// Returns whether all blockers completed.
    fn is_complete(&self) -> bool {
        self.remaining == 0
    }
}

/// Entity that defaults to [`Entity::PLACEHOLDER`].
struct OptEntity(Entity);

impl Default for OptEntity {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

/// System parameter for registering blocking tasks.
#[derive(SystemParam)]
pub struct BarrierBlocker<'w, 's, M: Component> {
    barrier: Query<'w, 's, (Entity, &'static mut Barrier), With<M>>,
    entity: Local<'s, OptEntity>,
    completed: Local<'s, bool>,
}

impl<'w, 's, M: Component> BarrierBlocker<'w, 's, M> {
    /// Checks whether this blocker is completed.
    pub fn is_completed(&self) -> bool {
        self.is_registered() && *self.completed
    }

    /// Checks whether this blocker is registered.
    pub fn is_registered(&self) -> bool {
        self.entity.0 == self.barrier.single().0
    }

    /// Registers the blocker in the barrier.
    pub fn register(&mut self) {
        if !self.is_registered() {
            let (entity, mut barrier) = self.barrier.single_mut();
            barrier.register();
            *self.entity = OptEntity(entity);
            *self.completed = false;
        } else {
            panic!("Barrier already registered.");
        }
    }

    /// Completes the blocker.
    pub fn complete(&mut self) {
        if !self.is_registered() {
            panic!("Barrier blocker not registered.");
        }
        if !*self.completed {
            self.barrier.single_mut().1.complete();
            *self.completed = true;
        } else {
            panic!("Barrier cannot be completed twice.")
        }
    }
}

/// System parameter for waiting for barrier to finish.
#[derive(SystemParam)]
pub struct BarrierAwaiter<'w, 's, M: Component> {
    barrier: Query<'w, 's, &'static Barrier, With<M>>,
}

impl<'w, 's, M: Component> BarrierAwaiter<'w, 's, M> {
    /// Returns true if all registered blockers completed.
    pub fn is_completed(&self) -> bool {
        self.barrier.single().is_complete()
    }
}
