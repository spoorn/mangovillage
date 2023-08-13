use bevy::prelude::Component;

#[derive(Component, Copy, Clone)]
pub struct PlayerData {
    pub id: u32,
    pub handle_id: u8,
}
