use std::collections::HashSet;
use std::time::Duration;

use bevy::app::App;
use bevy::prelude::{Commands, Entity, error, info, Plugin, Query, Res, ResMut};
use bevy::utils::HashMap;
use durian::{PacketManager, register_receive, register_send, ServerConfig};
use rand::Rng;

use crate::common::components::Position;
use crate::common::util;
use crate::networking::client_packets::{Disconnect, DisconnectPacketBuilder, Move, MovePacketBuilder};
use crate::networking::server_packets::{SpawnAck, UpdatePlayerPositions};
use crate::player::components::Player;
use crate::player::spawn_player;
use crate::server::resources::{ServerInfo, ServerPacketManager};
use crate::world::LEVEL_IIDS;

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
            .add_system(accept_new_player)
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

/// Adds new players to player pool
fn accept_new_player(mut commands: Commands, mut players_query: Query<(&Player, &mut Position, Entity)>, mut manager: ResMut<ServerPacketManager>) {
    let clients = manager.get_client_connections();
    let client_ids: HashSet<&u32> = clients.iter().map(|(_addr, id)| id).collect();
    
    let mut removed_players = Vec::new();
    // TODO: there has to be a faster way to do this than creating a map every iteration?  Can use a set too
    let mut players = HashMap::new();
    for (player, position, entity) in players_query.iter_mut() {
        players.insert(player.id, position);
        if !client_ids.contains(&player.id) {
            removed_players.push((player.id, entity));
        }
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
        spawn_player(&mut commands, None, *id, (148.0, 88.0), false);
        if let Err(e) = manager.send_to(addr, SpawnAck { id: *id, level_iid: LEVEL_IIDS[rand::thread_rng().gen_range(0..LEVEL_IIDS.len())].to_string() }) {
            error!("[server] Failed to send SpawnAck to addr={}.  Error: {}", addr, e);
        }
    }
    
    for (id, entity) in removed_players.into_iter() {
        info!("[server] Despawning player with id={}", id);
        commands.entity(entity).despawn();
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