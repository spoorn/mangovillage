use crate::player::component::ServerPlayer;
use bevy::prelude::{info, Commands};

pub mod component;

pub fn spawn_player(commands: &mut Commands, addr: String, id: u32) {
    info!("[server] Spawning player with addr={}, id={}", addr, id);
    commands.spawn(ServerPlayer { id, addr });
}
