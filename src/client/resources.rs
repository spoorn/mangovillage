use std::ops::{Deref, DerefMut};
use bevy::prelude::Resource;
use durian::PacketManager;

#[derive(Resource)]
pub struct ClientId {
    pub id: u32,
    // TODO: temporary, until logic in client.rs is fixed.  True if was set, False if still waiting to be set
    pub set: bool
}

#[derive(Resource)]
pub struct ClientInfo {
    pub client_addr: String,
    pub server_addr: String
}

#[derive(Resource)]
pub struct ClientPacketManager {
    pub manager: PacketManager
}

impl Deref for ClientPacketManager {
    type Target = PacketManager;

    fn deref(&self) -> &Self::Target {
        &self.manager
    }
}

impl DerefMut for ClientPacketManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.manager
    }
}