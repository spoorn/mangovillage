use durian::bincode_packet;
use crate::resource::LevelInfo;

#[bincode_packet]
pub struct ConnectAck;

#[bincode_packet]
pub struct SpawnScene {
    pub level: LevelInfo
}