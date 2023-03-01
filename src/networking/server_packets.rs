use durian::bincode_packet;
use serde::{Deserialize, Serialize};

#[bincode_packet]
pub struct SpawnPlayer {
    // TODO: allow custom sprites
    pub position: PlayerPosition
}

#[bincode_packet]
pub struct UpdatePositions {
    pub positions: Vec<PlayerPosition>
}

#[derive(Serialize, Deserialize)]
pub struct PlayerPosition {
    pub id: u32,
    pub position: (u32, u32)
}