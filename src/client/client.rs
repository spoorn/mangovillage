use std::time::Duration;
use bevy::app::App;
use bevy::prelude::{Commands, info, Plugin, Res};
use durian::{ClientConfig, PacketManager, register_receive, register_send};
use crate::client::resources::{ClientInfo, ClientPacketManager};
use crate::common::util;
use crate::networking::client_packets::Move;
use crate::networking::server_packets::{UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder};

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
    let receives = util::validate_results(true, register_receive!(manager, (UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder)));
    let sends = util::validate_results(true, register_send!(manager, Move));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    let mut client_config = ClientConfig::new(client_info.client_addr.clone(), client_info.server_addr.clone(), 1, 1);
    // Server sends keep alive packets
    client_config.with_keep_alive_interval(Duration::from_secs(30)).with_idle_timeout(Duration::from_secs(60));
    manager.init_client(client_config).unwrap();
    commands.insert_resource(ClientPacketManager { manager });
    info!("[client] Initialized client");
}