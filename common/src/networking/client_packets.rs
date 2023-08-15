use durian::bincode_packet;

/// Connect to server
#[bincode_packet]
pub struct Connect;

/// For graceful disconnects
#[bincode_packet]
pub struct Disconnect;

/// Player movement
#[bincode_packet]
pub struct Movement {
    /// x, y
    pub translation: [f32; 2],
}
