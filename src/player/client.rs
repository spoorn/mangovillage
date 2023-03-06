use std::collections::HashSet;

use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Entity, error, info, Input, KeyCode, Plugin, Query, Res, ResMut, Sprite, SpriteBundle, SystemSet, Transform, Vec2};
use bevy::utils::HashMap;

use crate::client::resources::{ClientId, ClientPacketManager};
use crate::common::components::Position;
use crate::common::Direction;
use crate::networking::client_packets::Move;
use crate::networking::server_packets::{UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder};
use crate::player::components::{ClientPlayer, Me};
use crate::state::client::ClientState;

pub struct PlayerClientPlugin;
impl Plugin for PlayerClientPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(ClientState::Running)
                .with_system(movement_input)
                .with_system(update_players)
        );
    }
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
        // TODO: only send if different from previous direction or stopped moving
        if let Err(e) = manager.send(Move { dir }) {
            error!("[client] Could not send Move packet: {}", e);
        }
    }
}

fn update_players(mut commands: Commands, mut players_query: Query<(&ClientPlayer, &mut Position, Entity)>, mut manager: ResMut<ClientPacketManager>, asset_server: Res<AssetServer>, client_id: Res<ClientId>) {
    let update_players = manager.received::<UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder>(false).unwrap();
    if let Some(update_players) = update_players {
        // We only care about the last update
        if let Some(last) = update_players.last() {
            // TODO: there has to be a faster way to do this than creating a map every iteration?
            let mut players = HashMap::new();
            let mut player_ids = HashSet::new();
            for (player, position, entity) in players_query.iter_mut() {
                players.insert(player.id, (position, entity));
                player_ids.insert(player.id);
            }
            
            // TODO: Would it be faster to handle a Despawn packet instead of looping through here?
            let mut server_players = HashSet::new();
            for player in last.positions.iter() {
                server_players.insert(player.id);
                if let Some((p, _entity)) = players.get_mut(&player.id) {
                    p.x = player.position.0;
                    p.y = player.position.1;
                } else {
                    // New player
                    spawn_player(&mut commands, &asset_server, player.id, player.position, player.id == client_id.id);
                }
            }
            
            // Remove despawned players
            for removed_player in player_ids.difference(&server_players) {
                let (pos, entity) = players.get(removed_player).unwrap();
                info!("[client] Despawning player with id={} at position=({}, {})", removed_player, pos.x, pos.y);
                commands.entity(*entity).despawn();
            }
        }
    }
    
    // TODO: handle removed players, can probably optimize above a bit if we do this
}

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, id: u32, position: (f32, f32), is_self: bool) {
    info!("[client] Spawning new player at ({}, {})", position.0, position.1);
    let mut player_spawn = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            texture: asset_server.load("icon/test.png"),
            ..default()
        });
    player_spawn
        .insert(ClientPlayer { id })
        .insert(Position { x: position.0, y: position.1 });

    if is_self {
        player_spawn.insert(Me);
    }
}