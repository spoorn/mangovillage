use std::time::Duration;

use bevy::app::App;
use bevy::prelude::{Commands, info, Plugin, Query, Res};
use bevy::utils::HashMap;
use durian::{PacketManager, register_receive, register_send, ServerConfig};

use crate::common::components::Position;
use crate::common::util;
use crate::networking::client_packets::{Move, MovePacketBuilder};
use crate::networking::server_packets::UpdatePlayerPositions;
use crate::player::components::Player;
use crate::player::spawn_player;
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
            .add_system(accept_new_player);
    }
}

fn init_server(mut commands: Commands, server_info: Res<ServerInfo>) {
    let mut manager = PacketManager::new();
    // register server side packets
    let receives = util::validate_results(false, register_receive!(manager, (Move, MovePacketBuilder)));
    let sends = util::validate_results(false, register_send!(manager, UpdatePlayerPositions));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    let mut server_config = ServerConfig::new(server_info.server_addr.clone(), 0, None, 1, 1);
    server_config.with_keep_alive_interval(Duration::from_secs(30)).with_idle_timeout(None);
    manager.init_server(server_config).unwrap();
    commands.insert_resource(ServerPacketManager { manager });
    info!("[server] Initialized server")
}

/// Adds new players to player pool
fn accept_new_player(mut commands: Commands, mut players_query: Query<(&Player, &mut Position)>, manager: Res<ServerPacketManager>) {
    let clients = manager.get_client_connections();
    
    // TODO: there has to be a faster way to do this than creating a map every iteration?  Can use a set too
    let mut players = HashMap::new();
    for (player, position) in players_query.iter_mut() {
        players.insert(player.id, position);
    }
        
    // TODO: handle removed players
    let mut new_players: Vec<(&String, &u32)> = Vec::new();
    if clients.len() != players.len() {
        for (addr, id) in clients.iter() {
            if !players.contains_key(id) {
                new_players.push((addr, id));
            }
        }
    }
    
    for (addr, id) in new_players.into_iter() {
        info!("[server] Found new player with addr={}, id={}", addr, id);
        spawn_player(&mut commands, None, *id, (0.0, 0.0));
    }
}