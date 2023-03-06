use std::time::Duration;

use bevy::app::App;
use bevy::prelude::{Commands, info, Plugin, Res, ResMut};
use durian::{PacketManager, register_receive, register_send, ServerConfig};

use crate::common::util;
use crate::networking::client_packets::{Disconnect, DisconnectPacketBuilder, Move, MovePacketBuilder};
use crate::networking::server_packets::{SpawnAck, UpdatePlayerPositions};
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
            .add_startup_system(init_server)
            .add_system(handle_leaves);
    }
}

fn init_server(mut commands: Commands, server_info: Res<ServerInfo>) {
    let mut manager = PacketManager::new();
    // register server side packets
    let receives = util::validate_results(false, register_receive!(manager, (Move, MovePacketBuilder), (Disconnect, DisconnectPacketBuilder)));
    let sends = util::validate_results(false, register_send!(manager, UpdatePlayerPositions, SpawnAck));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    let mut server_config = ServerConfig::new(server_info.server_addr.clone(), 0, None, 2, 2);
    server_config.with_keep_alive_interval(Duration::from_secs(30)).with_idle_timeout(Some(Duration::from_secs(60)));
    manager.init_server(server_config).unwrap();
    commands.insert_resource(ServerPacketManager { manager });
    info!("[server] Initialized server")
}

fn handle_leaves(mut manager: ResMut<ServerPacketManager>) {
    let leave_packets = manager.received_all::<Disconnect, DisconnectPacketBuilder>(false).unwrap();
    for (addr, leaves) in leave_packets {
        if let Some(leaves) = leaves {
            if !leaves.is_empty() {
                info!("[server] Client with addr={} has disconnected", addr);
                if let Err(e) = manager.close_connection(addr.clone()) {
                    info!("[server] Could not close connection with addr={}.  Error: {}", addr, e);
                }
            }
        }
    }
}