use durian::bincode_packet;
use serde::{Deserialize, Serialize};

/// Ack to new players when they join
#[bincode_packet]
pub struct SpawnAck {
    // clientId of the player, so the client can keep track of which player is themselves
    pub id: u32
}

#[bincode_packet]
#[derive(Debug)]
pub struct UpdatePlayerPositions {
    pub positions: Vec<PlayerPosition>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerPosition {
    pub id: u32,
    pub position: (f32, f32)
}