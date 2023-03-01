use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, error, Input, KeyCode, Plugin, Res, ResMut, Sprite, Vec2};
use bevy::sprite::SpriteBundle;
use crate::client::resources::ClientPacketManager;
use crate::common::Direction;
use crate::networking::client_packets::Move;

pub struct PlayerClientPlugin;
impl Plugin for PlayerClientPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(movement_input);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        texture: asset_server.load("icon/test.png"),
        ..default()
    });
}

fn movement_input(keys: Res<Input<KeyCode>>, mut manager: ResMut<ClientPacketManager>) {
    let dir: Option<Direction> = if keys.pressed(KeyCode::Left) {
        Some(Direction::Left)
    } else if keys.pressed(KeyCode::Down) {
        Some(Direction::Down)
    } else if keys.pressed(KeyCode::Up) {
        Some(Direction::Up)
    } else if keys.pressed(KeyCode::Right) {
        Some(Direction::Right)
    } else {
        None
    };
    
    if let Some(dir) = dir {
        if let Err(e) = manager.send(Move { dir }) {
            error!("[client] Could not send Move packet: {}", e);
        }
    }
}