use durian::bincode_packet;
use crate::common::Direction;

#[bincode_packet]
pub struct Move {
    pub dir: Direction
}

#[bincode_packet]
pub struct Disconnect;