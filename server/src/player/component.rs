use bevy::prelude::{Bundle, Component};

use mangovillage_common::physics::component::ColliderBundle;
use mangovillage_common::player::component::PlayerData;

#[derive(Bundle)]
pub struct ServerPlayerBundle {
    pub server_player: ServerPlayer,
    pub player_data: PlayerData,
    pub colliders: ColliderBundle,
}

#[derive(Component)]
pub struct ServerPlayer {
    pub addr: String,
}
