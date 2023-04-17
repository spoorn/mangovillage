use bevy::app::App;
use bevy::prelude::{info, Plugin, Query, Res, Time, Transform, Vec3, With};
use bevy_rapier3d::math::Vect;
use bevy_rapier3d::prelude::Velocity;

use crate::common::components::Position;
use crate::common::Direction;
use crate::player::components::{ClientPlayer, ServerPlayer};
use crate::world::resources::{Map, PortalInfo, World};

pub mod client;
pub mod server;
pub mod resources;
pub mod components;

pub struct PlayerCommonPlugin;
impl Plugin for PlayerCommonPlugin {
    
    fn build(&self, app: &mut App) {
    }
}

// Per second
const PLAYER_MOVE_SPEED: f32 = 1.0;

/// Handle an entity's movement
/// 
/// Optionally returns the new Level to load for the player if they should change levels
/// 
/// Note: The portal checking logic assumes no 2 portals are right next to each other or overlaid
/// TODO: Optimize portal checking logic
fn handle_move(direction: Direction, player: &mut ServerPlayer, velocity: &mut Velocity, transform: &mut Transform, current_map: &String, world: &Res<World>) -> Option<String> {
    let movement = PLAYER_MOVE_SPEED; //PLAYER_MOVE_SPEED * time.delta_seconds();
    let map = world.maps.get(current_map).unwrap();
    
    // let x = &mut transform.translation.x;
    // let y = &mut transform.translation.y;
    let x = &mut transform.translation.clone().x;
    let y = &mut transform.translation.clone().y;
    
    // Check if player started in a portal
    let prev_was_in_portal = player.was_in_portal.clone();
    let mut in_portal = false;
    for PortalInfo([x1, x2, y1, y2], destination, link) in &world.maps.get(current_map).unwrap().portals {
        if *x >= *x1 && *x <= *x2 && *y >= *y1 && *y <= *y2 {
            in_portal = true;
            if !prev_was_in_portal {
                info!("[server] Player at position=({}, {}) in Level={} warped to {}", *x, *y, current_map, destination);
                for PortalInfo([d_x1, d_x2, d_y1, d_y2], _d_destination, d_link) in &world.maps.get(destination).unwrap().portals {
                    if d_link == link {
                        // TODO: make sure player is grounded
                        *x = (d_x1 + d_x2) / 2.0;
                        *y = (d_y1 + d_y2) / 2.0;
                        velocity.linvel = Vect::ZERO;
                    }
                }
                player.was_in_portal = true;
                return Some(destination.clone());
            }
            break;
        }
    }
    player.was_in_portal = in_portal;
    
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