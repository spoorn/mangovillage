use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_rapier3d::prelude::*;

use crate::common::components::{ColliderBundle, Position};
use crate::common::Direction;
use crate::networking::client_packets::{Move, MovePacketBuilder};
use crate::networking::server_packets::{LevelInfo, PlayerInfo, SpawnAck, UpdatePlayerPositions};
use crate::player::components::{ServerPlayer, ServerPlayerBundle};
use crate::server::resources::ServerPacketManager;
use crate::state::server::ServerState;
use crate::world::components::WorldComponent;
use crate::world::resources::{Map, PortalInfo, World};

// Per second
const PLAYER_MOVE_SPEED: f32 = 1.0;

pub struct PlayerServerPlugin;
impl Plugin for PlayerServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_systems((send_player_positions, handle_player_move, accept_new_player).in_set(OnUpdate(ServerState::Running)));
    }
}

/// Adds new players to player pool
fn accept_new_player(mut commands: Commands, mut players_query: Query<(&ServerPlayer, &mut Position, Entity)>, mut manager: ResMut<ServerPacketManager>, asset_server: Res<AssetServer>) {
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
        let level_handle = "models/volcano_island_lowpoly/scene.gltf#Scene0";
        let world_component = WorldComponent { level_handle: level_handle.to_string() };
        // TODO: Randomize exact pixel placement of player spawn
        let position = Position::new(0.0, 0.0, 15.0);
        spawn_player(&mut commands, addr.clone(), *id, position, world_component, "models/owl/scene.gltf#Scene0", &asset_server);
        // TODO: remove this, should have only one place where scene transform is determined
        let level = LevelInfo {
            handle_id: level_handle.to_string(),
            scene_transform: [0.0, 0.0, -20.0, std::f32::consts::PI / 2.0],
            scale: 0.005
        };
        if let Err(e) = manager.send_to(addr, SpawnAck { id: *id, level }) {
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
fn send_player_positions(mut players: Query<(&ServerPlayer, &Transform, &WorldComponent)>, mut manager: ResMut<ServerPacketManager>) {
    // Level Id -> (Client addresses in the Level, player positions in the Level)
    let mut pps: HashMap<String, (Vec<&String>, Vec<PlayerInfo>)> = HashMap::new();
    for (player, transform, world_component) in players.iter_mut() {
        // TODO: Send local pos, instead of global pos
        let entry = pps.entry(world_component.level_handle.clone()).or_insert((Vec::new(), Vec::new()));
        entry.0.push(&player.addr);
        entry.1.push(PlayerInfo {
            handle_id: player.handle_id.to_string(),
            id: player.id,
            local_pos: (transform.translation.x, transform.translation.y, transform.translation.z)
        });
    }
    
    for (_level_iid, (addrs, pps)) in pps {
        for addr in addrs {
            if let Err(e) = manager.send_to(addr, UpdatePlayerPositions { players: pps.to_vec() }) {
                warn!("[server] Could not send updated player positions to addr={}.  They may have disconnected.  Error: {}", addr, e);
            }
        }
    }
}

fn handle_player_move(mut players_query: Query<(&mut ServerPlayer, &mut Velocity, &mut Transform, &mut WorldComponent)>, mut manager: ResMut<ServerPacketManager>) {
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
                        if let Some(change_level) = handle_move(last.dir, player, velocity, transform) {
                            // if let Err(e) = manager.send_to(addr, ChangeLevel { level_iid: change_level.clone() }) {
                            //     warn!("[server] Could not send ChangeLevel to addr={}. Error: {}", addr, e);
                            // } else {
                            //     world_component.level_iid = change_level;
                            // }
                        }
                    }
                }
            }
        }
    }
}

/// Handle an entity's movement
///
/// Optionally returns the new Level to load for the player if they should change levels
///
/// Note: The portal checking logic assumes no 2 portals are right next to each other or overlaid
/// TODO: Optimize portal checking logic
fn handle_move(direction: Direction, player: &mut ServerPlayer, velocity: &mut Velocity, transform: &mut Transform) -> Option<String> {
    let movement = PLAYER_MOVE_SPEED; //PLAYER_MOVE_SPEED * time.delta_seconds();
    //let map = world.maps.get(current_map).unwrap();

    // let x = &mut transform.translation.x;
    // let y = &mut transform.translation.y;
    let x = &mut transform.translation.clone().x;
    let y = &mut transform.translation.clone().y;

    // Check if player started in a portal
    // let prev_was_in_portal = player.was_in_portal.clone();
    // let mut in_portal = false;
    // for PortalInfo([x1, x2, y1, y2], destination, link) in &world.maps.get(current_map).unwrap().portals {
    //     if *x >= *x1 && *x <= *x2 && *y >= *y1 && *y <= *y2 {
    //         in_portal = true;
    //         if !prev_was_in_portal {
    //             info!("[server] Player at position=({}, {}) in Level={} warped to {}", *x, *y, current_map, destination);
    //             for PortalInfo([d_x1, d_x2, d_y1, d_y2], _d_destination, d_link) in &world.maps.get(destination).unwrap().portals {
    //                 if d_link == link {
    //                     // TODO: make sure player is grounded
    //                     *x = (d_x1 + d_x2) / 2.0;
    //                     *y = (d_y1 + d_y2) / 2.0;
    //                     velocity.linvel = Vect::ZERO;
    //                 }
    //             }
    //             player.was_in_portal = true;
    //             return Some(destination.clone());
    //         }
    //         break;
    //     }
    // }
    // player.was_in_portal = in_portal;

    match direction {
        Direction::Left => {
            // if *x <= map.bounds[0] {
            //     if let Some(neighbor) = find_neighbor("w", map) {
            //         *x = world.maps.get(&neighbor).unwrap().bounds[1];
            //         velocity.linvel = Vect::ZERO;
            //         return Some(neighbor);
            //     } else {
            //         *x = map.bounds[0];
            //     }
            // } else {
            //     velocity.linvel.x = -movement;
            // }
            velocity.linvel.x = -movement;
        }
        Direction::Up => {
            // if *y >= map.bounds[3] {
            //     if let Some(neighbor) = find_neighbor("n", map) {
            //         *y = world.maps.get(&neighbor).unwrap().bounds[2];
            //         velocity.linvel = Vect::ZERO;
            //         return Some(neighbor);
            //     } else {
            //         *y = map.bounds[3];
            //     }
            // } else {
            //     velocity.linvel.y = movement;
            // }
            velocity.linvel.y = movement;
        }
        Direction::Right => {
            // if *x >= map.bounds[1] {
            //     if let Some(neighbor) = find_neighbor("e", map) {
            //         *x = world.maps.get(&neighbor).unwrap().bounds[0];
            //         velocity.linvel = Vect::ZERO;
            //         return Some(neighbor);
            //     } else {
            //         *x = map.bounds[1];
            //     }
            // } else {
            //     velocity.linvel.x = movement;
            // }
            velocity.linvel.x = movement;
        }
        Direction::Down => {
            // if *y <= map.bounds[2] {
            //     if let Some(neighbor) = find_neighbor("s", map) {
            //         *y = world.maps.get(&neighbor).unwrap().bounds[3];
            //         velocity.linvel = Vect::ZERO;
            //         return Some(neighbor);
            //     } else {
            //         *y = map.bounds[2];
            //     }
            // } else {
            //     velocity.linvel.y = -movement;
            // }
            velocity.linvel.y = -movement;
        }
    }

    None
}

/// Finds a Map's neighbor based on the direction
#[inline]
fn find_neighbor(dir: &str, map: &Map) -> Option<String> {
    for (direction, neighbor) in &map.neighbors {
        if direction == dir {
            return Some(neighbor.clone());
        }
    }
    None
}

fn spawn_player(commands: &mut Commands, addr: String, id: u32, position: Position, world: WorldComponent, player_handle_id: &str, asset_server: &Res<AssetServer>) {
    info!("[server] Spawning new player in {} at {}", world.level_handle, position);
    // TODO: Change to headless
    let mut player_spawn = commands
        //.spawn(TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 10.0)));
        .spawn(SceneBundle {
            scene: asset_server.load(player_handle_id),
            transform: Transform::from_xyz(0.0, 0.0, 15.0).with_scale(Vec3::splat(0.05)),
            ..default()
        });
    player_spawn
        .insert(ServerPlayerBundle {
            player: ServerPlayer { handle_id: player_handle_id.to_string(), id, addr, was_in_portal: false },
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(12.0, 12.0, 12.0),
                rigid_body: RigidBody::Dynamic,
                // damping: Damping {
                //     linear_damping: 0.0,
                //     angular_damping: 0.0
                // },
                // friction: Friction {
                //     coefficient: 0.0,
                //     combine_rule: CoefficientCombineRule::Min,
                // },
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..default()
            }
        })
        .insert(Sleeping {
            linear_threshold: 0.05,
            ..default()
        })
        .insert(position)
        .insert(world);
}