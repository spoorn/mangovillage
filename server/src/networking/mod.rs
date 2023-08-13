use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashSet;
use durian::{register_receive, register_send, PacketManager, ServerConfig};

use mangovillage_common::networking::client_packets::{Connect, ConnectPacketBuilder, Disconnect, DisconnectPacketBuilder};
use mangovillage_common::networking::server_packets::{ConnectAck, Players, SpawnScene};
use mangovillage_common::resource::LevelInfo;
use mangovillage_common::util;

use crate::networking::resource::{ServerInfo, ServerPacketManager};
use crate::player;
use crate::player::component::ServerPlayer;
use crate::state::ServerState;

pub mod resource;

pub struct ServerPlugin {
    pub server_addr: String,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerInfo { server_addr: self.server_addr.clone() })
            .add_systems(Startup, init_server)
            .add_systems(Update, transition_load_world.run_if(in_state(ServerState::StartUp)))
            .add_systems(Update, (handle_leaves, handle_connects).run_if(in_state(ServerState::Running)));
    }
}

fn init_server(mut commands: Commands, server_info: Res<ServerInfo>) {
    let mut manager = PacketManager::new();
    // register server side packets
    let receives =
        util::validate_register_results(false, register_receive!(manager, (Connect, ConnectPacketBuilder), (Disconnect, DisconnectPacketBuilder)));
    let sends = util::validate_register_results(false, register_send!(manager, ConnectAck, SpawnScene, Players));
    // TODO: better error handling
    if !receives {
        panic!("Failed to register all receive packets");
    }
    if !sends {
        panic!("Failed to register all send packets");
    }
    let mut server_config = ServerConfig::new(server_info.server_addr.clone(), 0, None, 2, 3);
    server_config.with_keep_alive_interval(Duration::from_secs(30));
    manager.init_server(server_config).unwrap();

    info!("[server] Initialized server");
    commands.insert_resource(ServerPacketManager { manager });
}

// TODO: sweep for clients that did not send legit Connect packet and disconnect them
fn handle_connects(mut manager: ResMut<ServerPacketManager>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let connect_packets = manager.received_all::<Connect, ConnectPacketBuilder>(false).unwrap();
    for (addr, leaves) in connect_packets.into_iter() {
        if matches!(leaves, Some(connects) if !connects.is_empty()) {
            info!("[server] Client with addr={} connected", addr);
            player::spawn_player(&mut commands, addr.clone(), manager.get_client_id(addr.clone()).unwrap(), &asset_server);
            let client_id = manager.get_client_id(&addr).unwrap();
            info!("Sending ConnectAck to client id={}, addr={}", client_id, addr);
            manager.send_to(&addr, ConnectAck { id: client_id }).unwrap();
            // TODO: refactor this out of here
            info!("[server] Sending SpawnScene command to client {}", addr);
            manager
                .send_to(
                    addr,
                    SpawnScene {
                        level: LevelInfo {
                            handle_id: "models/volcano_island_lowpoly/scene.gltf#Scene0".to_string(),
                            scene_transform: [0.0, 0.0, 0.0, std::f32::consts::PI / 2.0],
                            scale: 0.005,
                        },
                    },
                )
                .unwrap();
        }
    }
}

fn handle_leaves(mut manager: ResMut<ServerPacketManager>, mut commands: Commands, players_query: Query<(Entity, &ServerPlayer)>) {
    let leave_packets = manager.received_all::<Disconnect, DisconnectPacketBuilder>(false).unwrap();
    let mut players_to_remove = HashSet::new();

    for (addr, leaves) in leave_packets {
        if let Some(leaves) = leaves {
            if !leaves.is_empty() {
                info!("[server] Client with addr={} has disconnected", addr);
                if let Err(e) = manager.close_connection(addr.clone()) {
                    error!("[server] Could not close connection with addr={}.  Error: {}", addr, e);
                }
                players_to_remove.insert(addr.clone());
            }
        }
    }

    // Remove disconnected players
    for (entity, player) in &players_query {
        if players_to_remove.contains(&player.addr) {
            info!("[server] Removing player with addr={}", player.addr);
            commands.entity(entity).despawn();
        }
    }
}

/// Load in the world right away
fn transition_load_world(mut server_state: ResMut<NextState<ServerState>>) {
    info!("Transitioning state to LoadWorld");
    server_state.set(ServerState::LoadWorld);
}
