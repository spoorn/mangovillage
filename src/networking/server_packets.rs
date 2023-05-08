use bevy::prelude::Resource;
use durian::bincode_packet;
use serde::{Deserialize, Serialize};

/// Ack to new players when they join
#[bincode_packet]
pub struct SpawnAck {
    // clientId of the player, so the client can keep track of which player is themselves
    pub id: u32,
    // Which level to load when spawning in
    pub level: LevelInfo
}

// Can also be used as a Resource
// TODO: don't send handle_id in a packet, so we can copy efficiently?
#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct LevelInfo {
    pub handle_id: String,
    // x, y, z, x-rotation
    pub scene_transform: [f32; 4],
    pub scale: f32
}

#[bincode_packet]
#[derive(Debug)]
pub struct UpdatePlayerPositions {
    pub players: Vec<PlayerInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInfo {
    // TODO: Don't send this in a packet!
    pub handle_id: String,
    pub id: u32,
    pub local_pos: (f32, f32, f32)
}

#[bincode_packet]
pub struct ChangeLevel {
    pub handle_id: String
}