use bevy::app::App;
use bevy::prelude::{Commands, info, Plugin, Res};
use durian::{ClientConfig, PacketManager};
use crate::client::resources::{ClientInfo, ClientPacketManager};

pub struct ClientPlugin {
    pub client_addr: String,
    pub server_addr: String
}

impl Plugin for ClientPlugin {
    
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientInfo {
            client_addr: self.client_addr.clone(),
            server_addr: self.server_addr.clone()
        })
            .add_startup_system(init_client);
    }
}

fn init_client(mut commands: Commands, client_info: Res<ClientInfo>) {
    let mut manager = PacketManager::new();
    // register packets client-side
    manager.init_client(ClientConfig::new(client_info.client_addr.clone(), client_info.server_addr.clone(), 0, 0)).unwrap();
    commands.insert_resource(ClientPacketManager { manager });
    info!("[client] Initialized client");
}