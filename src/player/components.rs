use bevy::prelude::{Bundle, Component};

use crate::common::components::ColliderBundle;

#[derive(Component)]
pub struct ClientPlayer {
    pub id: u32
}

#[derive(Bundle)]
pub struct ServerPlayerBundle {
    pub player: ServerPlayer,
    pub collider_bundle: ColliderBundle
}

#[derive(Component)]
pub struct ServerPlayer {
    pub id: u32,
    pub addr: String,
    pub was_in_portal: bool
}

/// To track which player is self
#[derive(Component)]
pub struct Me;
