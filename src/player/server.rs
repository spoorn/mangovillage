use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::common::components::{ColliderBundle, Position};
use crate::networking::client_packets::{Move, MovePacketBuilder};
use crate::networking::server_packets::{ChangeLevel, PlayerPosition, SpawnAck, UpdatePlayerPositions};
use crate::player::components::{ServerPlayer, ServerPlayerBundle};
use crate::player::handle_move;
use crate::server::resources::ServerPacketManager;
use crate::state::server::ServerState;
use crate::world::components::WorldComponent;
use crate::world::LEVEL_IIDS;
use crate::world::resources::World;

pub struct PlayerServerPlugin;
impl Plugin for PlayerServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_systems((send_player_positions, handle_player_move, accept_new_player).in_set(OnUpdate(ServerState::Running)));
    }
}

/// Adds new players to player pool
fn accept_new_player(mut commands: Commands, mut players_query: Query<(&ServerPlayer, &mut Position, Entity)>, mut manager: ResMut<ServerPacketManager>, world: Res<World>) {
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
        let level_iid = LEVEL_IIDS[rand::thread_rng().gen_range(0..LEVEL_IIDS.len())].to_string();
        let world_component = WorldComponent { level_iid: level_iid.clone() };
        // TODO: Randomize exact pixel placement of player spawn
        spawn_player(&mut commands, addr.clone(), *id, world.maps.get(&level_iid).unwrap().player_spawn, world_component);
        if let Err(e) = manager.send_to(addr, SpawnAck { id: *id, level_iid }) {
            error!("[server] Failed to send SpawnAck to addr={}.  Error: {}", addr, e);
        }
    }

    for (id, entity) in removed_players.into_iter() {
        info!("[server] Despawning player with id={}", id);
        commands.entity(entity).despawn();
    }
}

// TODO: optimize: Don't send all player positions constantly, only changed
// TODO: optimize: cache World -> Player data instead of querying every iteration like this
// TODO: optimize: Send player positions for each world independently in parallel
fn send_player_positions(mut players: Query<(&ServerPlayer, &Transform, &WorldComponent)>, mut manager: ResMut<ServerPacketManager>, world: Res<World>) {
    // Level iid -> (Client addresses in the Level, player positions in the Level)
    let mut pps: HashMap<String, (Vec<&String>, Vec<PlayerPosition>)> = HashMap::new();
    for (player, transform, world_component) in players.iter_mut() {
        let entry = pps.entry(world_component.level_iid.to_string()).or_insert((Vec::new(), Vec::new()));
        let world_coords = world.maps.get(world_component.level_iid.as_str()).unwrap().world_coords;
        entry.0.push(&player.addr);
        entry.1.push(PlayerPosition {
            id: player.id,
            local_pos: (transform.translation.x - world_coords.0, transform.translation.y - world_coords.1)
        });
    }
    
    for (_level_iid, (addrs, pps)) in pps {
        for addr in addrs {
            if let Err(e) = manager.send_to(addr, UpdatePlayerPositions { positions: pps.to_vec() }) {
                warn!("[server] Could not send updated player positions to addr={}.  They may have disconnected.  Error: {}", addr, e);
            }
        }
    }
}

fn handle_player_move(mut players_query: Query<(&mut ServerPlayer, &mut Velocity, &mut Transform, &mut WorldComponent)>, mut manager: ResMut<ServerPacketManager>, world: Res<World>) {
    let move_packets = manager.received_all::<Move, MovePacketBuilder>(false).unwrap();
    
    if !move_packets.is_empty() {
        // TODO: there has to be a faster way to do this than creating a map every iteration?  Can use a set too
        let mut players = HashMap::new();
        for (player, velocity, transform, world_component) in players_query.iter_mut() {
            players.insert(player.id, (player, velocity, transform, world_component));
        }
        
        for (addr, moves) in move_packets.iter() {
            if let Some(moves) = moves {
                // We only care about the last movement from the player
                //for last in moves.iter() {
                if let Some(last) = moves.last() {
                    let player_id = manager.get_client_id(addr).unwrap();
                    if let Some((ref mut player, ref mut velocity, ref mut transform, ref mut world_component)) = players.get_mut(&player_id) {
                        if let Some(change_level) = handle_move(last.dir, player, velocity, transform, &world_component.level_iid, &world) {
                            if let Err(e) = manager.send_to(addr, ChangeLevel { level_iid: change_level.clone() }) {
                                warn!("[server] Could not send ChangeLevel to addr={}. Error: {}", addr, e);
                            } else {
                                world_component.level_iid = change_level;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_player(commands: &mut Commands, addr: String, id: u32, position: Position, world: WorldComponent) {
    info!("[server] Spawning new player in {} at {}", world.level_iid, position);
    let mut player_spawn = commands
        .spawn(TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 10.0)));
    player_spawn
        .insert(ServerPlayerBundle {
            player: ServerPlayer { id, addr, was_in_portal: false },
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(12.0, 12.0),
                rigid_body: RigidBody::Dynamic,
                damping: Damping {
                    linear_damping: 100.0,
                    angular_damping: 0.0
                },
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..default()
            }
        })
        .insert(position)
        .insert(world);
}