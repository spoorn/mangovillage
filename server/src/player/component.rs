use bevy::prelude::Component;

#[derive(Component)]
pub struct ServerPlayer {
    pub id: u32,
    pub addr: String,
}
