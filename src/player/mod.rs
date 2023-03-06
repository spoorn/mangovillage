use bevy::app::App;
use bevy::prelude::{Plugin, Query, Res, Time, Transform, Vec3, With};

use crate::common::components::Position;
use crate::common::Direction;
use crate::player::components::ClientPlayer;
use crate::world::resources::{Map, World};

pub mod client;
pub mod server;
pub mod resources;
pub mod components;

pub struct PlayerCommonPlugin;
impl Plugin for PlayerCommonPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_system(transform_positions);
    }
}

pub fn transform_positions(mut query: Query<(&Position, &mut Transform), With<ClientPlayer>>) {
    for (pos, mut trans) in query.iter_mut() {
        if pos.x != trans.translation.x || pos.y != trans.translation.y {  // Avoid new instantiations if possible
            trans.translation = Vec3::new(pos.x, pos.y, trans.translation.z);
        }
    }
}

// Per second
const PLAYER_MOVE_SPEED: f32 = 500.0;

/// Handle an entity's movement
/// 
/// Optionally returns the new Level to load for the player if they should change levels
fn handle_move(time: &Res<Time>, direction: Direction, position: &mut Position, current_map: &String, world: &Res<World>) -> Option<String> {
    let movement = PLAYER_MOVE_SPEED * time.delta_seconds();
    let map = world.maps.get(current_map).unwrap();
    match direction {
        Direction::Left => { 
            if position.x <= map.bounds[0] {
                if let Some(neighbor) = find_neighbor("w", map) {
                    position.x = world.maps.get(&neighbor).unwrap().bounds[1];
                    return Some(neighbor);
                }
            } else {
                position.x -= movement;
            }
        }
        Direction::Up => {
            if position.y >= map.bounds[3] {
                if let Some(neighbor) = find_neighbor("n", map) {
                    position.y = world.maps.get(&neighbor).unwrap().bounds[2];
                    return Some(neighbor);
                }
            } else {
                position.y += movement;
            }
        }
        Direction::Right => {
            if position.x >= map.bounds[1] {
                if let Some(neighbor) = find_neighbor("e", map) {
                    position.x = world.maps.get(&neighbor).unwrap().bounds[0];
                    return Some(neighbor);
                }
            } else {
                position.x += movement;
            }
        }
        Direction::Down => {
            if position.y <= map.bounds[2] {
                if let Some(neighbor) = find_neighbor("s", map) {
                    position.y = world.maps.get(&neighbor).unwrap().bounds[3];
                    return Some(neighbor);
                }
            } else {
                position.y -= movement;
            }
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