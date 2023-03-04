use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, info, Plugin, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy_ecs_ldtk::LdtkWorldBundle;

use crate::common::components::Position;
use crate::common::Direction;
use crate::player::components::{Me, Player};

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

pub fn spawn_player(commands: &mut Commands, asset_server: Option<&Res<AssetServer>>, id: u32, position: (f32, f32), is_self: bool) {
    info!("Spawning new player at ({}, {})", position.0, position.1);
    let mut player_spawn = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            texture: if let Some(asset_server) = asset_server { asset_server.load("icon/test.png") } else { DEFAULT_IMAGE_HANDLE.typed() },
            ..default()
        });
    player_spawn
        .insert(Player { id })
        .insert(Position { x: position.0, y: position.1 });
    
    if is_self {
        player_spawn.insert(Me);
    }
    
    if let Some(asset_server) = asset_server {
        commands.spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load("ldtk/test.ldtk"),
            // transform: Transform {
            //     //translation: Vec3::new(-500.0, -500.0, 1.0),
            //     scale: Vec3::new(2.0, 2.0, 1.0),
            //     ..default()
            // },
            ..default()
        });
    }
}

pub fn transform_positions(mut query: Query<(&Position, &mut Transform), With<Player>>) {
    for (pos, mut trans) in query.iter_mut() {
        if pos.x != trans.translation.x || pos.y != trans.translation.y {  // Avoid new instantiations if possible
            trans.translation = Vec3::new(pos.x, pos.y, trans.translation.z);
        }
    }
}

// Handle an entity's movement
fn handle_move(direction: Direction, position: &mut Position) {
    match direction {
        Direction::Left => { position.x -= 1.0; }
        Direction::Up => { position.y += 1.0; }
        Direction::Right => { position.x += 1.0; }
        Direction::Down => { position.y -= 1.0; }
    }
}