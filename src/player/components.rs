use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub id: u32
}

/// To track which player is self
#[derive(Component)]
pub struct Me;