use durian::bincode_packet;
use serde::{Deserialize, Serialize};

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