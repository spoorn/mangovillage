use bevy::app::App;
use bevy::prelude::{Commands, info, Plugin, Res};
use durian::{PacketManager, ServerConfig};

use crate::server::resources::{ServerInfo, ServerPacketManager};

pub struct ServerPlugin {
    pub server_addr: String
}

impl Plugin for ServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerInfo {
            server_addr: self.server_addr.clone(),
            want_num_clients: 1
        })
            .add_startup_system(init_server);
    }
}

fn init_server(mut commands: Commands, server_info: Res<ServerInfo>) {
    let mut manager = PacketManager::new();
    // register server side packets
    manager.init_server(ServerConfig::new(server_info.server_addr.clone(), 0, None, 0, 0)).unwrap();
    commands.insert_resource(ServerPacketManager { manager });
    info!("[server] Initialized server")
}