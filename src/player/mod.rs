use bevy::app::App;
use bevy::prelude::{Plugin, Query, Res, Time, Transform, Vec3, With};

use crate::common::components::Position;
use crate::common::Direction;
use crate::player::components::ClientPlayer;

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
const PLAYER_MOVE_SPEED: f32 = 100.0;

// Handle an entity's movement
fn handle_move(time: &Res<Time>, direction: Direction, position: &mut Position) {
    let movement = PLAYER_MOVE_SPEED * time.delta_seconds();
    match direction {
        Direction::Left => { position.x -= movement; }
        Direction::Up => { position.y += movement; }
        Direction::Right => { position.x += movement; }
        Direction::Down => { position.y -= movement; }
    }
}