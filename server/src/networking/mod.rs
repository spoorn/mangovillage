use std::time::Duration;
use bevy::prelude::*;
use durian::{PacketManager, register_receive, register_send, ServerConfig};
use mangovillage_common::networking::client_packets::{Connect, ConnectPacketBuilder, Disconnect, DisconnectPacketBuilder};
use mangovillage_common::networking::server_packets::ConnectAck;
use mangovillage_common::util;
use crate::networking::resource::{ServerInfo, ServerPacketManager};

mod resource;

pub struct ServerPlugin {
    pub server_addr: String
}

impl Plugin for ServerPlugin {

    fn build(&self, app: &mut App) {
        app.insert_resource(ServerInfo {
            server_addr: self.server_addr.clone()
        })
            .add_systems(Startup, init_server)
            .add_systems(Update, (handle_leaves, handle_connects));
    }
}

fn init_server(mut commands: Commands, server_info: Res<ServerInfo>) {
    let mut manager = PacketManager::new();
    // register server side packets
    let receives = util::validate_register_results(false, register_receive!(manager, (Connect, ConnectPacketBuilder), (Disconnect, DisconnectPacketBuilder)));
    let sends = util::validate_register_results(false, register_send!(manager, ConnectAck));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    let mut server_config = ServerConfig::new(server_info.server_addr.clone(), 0, None, 2, 1);
    server_config.with_keep_alive_interval(Duration::from_secs(30));
    manager.init_server(server_config).unwrap();
    
    info!("[server] Initialized server");
    commands.insert_resource(ServerPacketManager { manager });
}

fn handle_connects(mut manager: ResMut<ServerPacketManager>) {
    let connect_packets = manager.received_all::<Connect, ConnectPacketBuilder>(false).unwrap();
    for (addr, leaves) in connect_packets {
        if matches!(leaves, Some(connects) if !connects.is_empty()) {
            info!("[server] Client with addr={} connected", addr);
        }
    }
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