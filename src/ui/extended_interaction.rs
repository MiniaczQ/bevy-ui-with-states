use bevy::{prelude::*, ui::UiSystem};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, install);
    app.add_systems(PreUpdate, update.after(UiSystem::Focus));
}

fn install(mut commands: Commands, query: Query<(Entity, &Interaction), Added<Interaction>>) {
    for (entity, interaction) in query.iter() {
        commands
            .entity(entity)
            .insert(ExtendedInteration::new(*interaction));
    }
}

fn update(mut query: Query<(&mut ExtendedInteration, &Interaction), Changed<Interaction>>) {
    for (mut extended_interaction, interaction) in query.iter_mut() {
        extended_interaction.update(*interaction);
    }
}

/// Interaction that provides buffered information like just pressed/released.
#[derive(Component, Default, Debug)]
pub struct ExtendedInteration {
    previous: Interaction,
    current: Interaction,
}

impl ExtendedInteration {
    fn new(interaction: Interaction) -> Self {
        Self {
            previous: interaction,
            current: interaction,
        }
    }

    fn update(&mut self, interaction: Interaction) {
        self.previous = self.current;
        self.current = interaction;
    }

    pub fn is_none(&self) -> bool {
        matches!(self.current, Interaction::None)
    }

    pub fn is_hovering(&self) -> bool {
        self.current == Interaction::Hovered
    }

    pub fn is_pressing(&self) -> bool {
        self.current == Interaction::Pressed
    }

    pub fn just_pressed(&self) -> bool {
        self.previous != Interaction::Pressed && self.current == Interaction::Pressed
    }

    pub fn just_released(&self) -> bool {
        self.previous == Interaction::Pressed && self.current != Interaction::Pressed
    }
}
