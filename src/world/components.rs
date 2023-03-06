use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Component)]
pub struct WorldComponent {
    pub level_iid: String
}

// Player Spawn portal entities
#[derive(Component, Default)]
pub struct PlayerSpawn;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    pub player_spawn: PlayerSpawn
}