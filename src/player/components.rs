use bevy::prelude::Component;

#[derive(Component)]
pub struct ClientPlayer {
    pub id: u32
}

#[derive(Component)]
pub struct ServerPlayer {
    pub id: u32,
    pub addr: String
}

/// To track which player is self
#[derive(Component)]
pub struct Me;
