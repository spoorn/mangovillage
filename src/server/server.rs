use bevy::app::App;
use bevy::prelude::{Commands, info, Plugin, Res, ResMut};
use bevy::utils::HashMap;
use durian::{PacketManager, register_receive, register_send, ServerConfig};

use crate::common::util;
use crate::networking::client_packets::{Move, MovePacketBuilder};
use crate::networking::server_packets::{SpawnPlayer, UpdatePositions};
use crate::server::resources::{Player, Players, ServerInfo, ServerPacketManager};

pub struct ServerPlugin {
    pub server_addr: String
}

impl Plugin for ServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerInfo {
            server_addr: self.server_addr.clone(),
            want_num_clients: 1
        })
            .insert_resource(Players {
                players: HashMap::new()
            })
            .add_startup_system(init_server)
            .add_system(accept_new_player);
    }
}

fn init_server(mut commands: Commands, server_info: Res<ServerInfo>) {
    let mut manager = PacketManager::new();
    // register server side packets
    let receives = util::validate_results(false, register_receive!(manager, (Move, MovePacketBuilder)));
    let sends = util::validate_results(false, register_send!(manager, SpawnPlayer, UpdatePositions));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    manager.init_server(ServerConfig::new(server_info.server_addr.clone(), 0, None, 1, 2)).unwrap();
    commands.insert_resource(ServerPacketManager { manager });
    info!("[server] Initialized server")
}

fn accept_new_player(mut players: ResMut<Players>, manager: Res<ServerPacketManager>) {
    let clients = manager.get_client_connections();
    // TODO: handle removed players
    let mut new_players: Vec<(&String, &u32)> = Vec::new();
    if clients.len() != players.players.len() {
        for (addr, id) in clients.iter() {
            if !players.players.contains_key(id) {
                new_players.push((addr, id));
            }
        }
    }
    
    for (addr, id) in new_players.into_iter() {
        info!("[server] Found new player with addr={}, id={}", addr, id);
        players.players.insert(*id, Player { position: (0, 0) });
    }
}