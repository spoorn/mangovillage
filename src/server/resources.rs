use std::ops::{Deref, DerefMut};
use bevy::prelude::Resource;
use durian::PacketManager;

#[derive(Resource)]
pub struct ServerInfo {
    pub server_addr: String,
    pub want_num_clients: u8
}

#[derive(Resource)]
pub struct ServerPacketManager {
    pub manager: PacketManager
}

impl Deref for ServerPacketManager {
    type Target = PacketManager;

    fn deref(&self) -> &Self::Target {
        &self.manager
    }
}

impl DerefMut for ServerPacketManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.manager
    }
}