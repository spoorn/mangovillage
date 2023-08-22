use bevy::prelude::Component;
use durian::bincode_packet;
use serde::{Deserialize, Serialize};

use crate::resource::LevelInfo;

#[bincode_packet]
pub struct ConnectAck {
    /// Client's server ID
    pub id: u32,
}

#[bincode_packet]
#[derive(Debug)]
pub struct SpawnScene {
    pub level: LevelInfo,
}

#[bincode_packet]
pub struct Players {
    pub players: Vec<Player>,
}

// TODO: optimize so we can use Copy
#[derive(Component, Serialize, Deserialize, Copy, Clone)]
pub struct Player {
    /// Client's server ID
    pub id: u32,
    pub handle_id: u8,
    // x, y, z
    pub transform: [f32; 3],
    pub scale: f32,
}
