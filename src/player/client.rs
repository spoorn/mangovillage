use std::collections::HashSet;

use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Entity, error, info, Input, IntoSystemConfigs, KeyCode, OnUpdate, Plugin, Query, Res, ResMut, SceneBundle, Transform, Vec3, With};
use bevy::utils::HashMap;

use crate::client::resources::{ClientId, ClientPacketManager};
use crate::common::components::Position;
use crate::common::Direction;
use crate::networking::client_packets::Move;
use crate::networking::server_packets::{PlayerInfo, UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder};
use crate::player::components::{ClientPlayer, Me};
use crate::state::client::ClientState;

pub struct PlayerClientPlugin;
impl Plugin for PlayerClientPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_systems((movement_input, update_players, transform_positions).in_set(OnUpdate(ClientState::Running)));
    }
}

// Update bevy Transform from our Position
pub fn transform_positions(mut query: Query<(&Position, &mut Transform), With<ClientPlayer>>) {
    for (pos, mut trans) in query.iter_mut() {
        if pos.x != trans.translation.x || pos.y != trans.translation.y || pos.z != trans.translation.z {  // Avoid new instantiations if possible
            trans.translation = Vec3::new(pos.x, pos.y, pos.z);
        }
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

// Player position is a local position with respect to the Map
fn update_players(mut commands: Commands, mut players_query: Query<(&ClientPlayer, &mut Position, Entity)>, mut manager: ResMut<ClientPacketManager>, asset_server: Res<AssetServer>, client_id: Res<ClientId>) {
    let update_players = manager.received::<UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder>(false).unwrap();
    // We only care about the last update
    if let Some(update_players) = update_players.as_ref().and_then(|x| x.last()) {
        // TODO: there has to be a faster way to do this than creating a map every iteration?
        let mut players = HashMap::new();
        let mut player_ids = HashSet::new();
        for (player, position, entity) in players_query.iter_mut() {
            players.insert(player.id, (position, entity));
            player_ids.insert(player.id);
        }
        
        // TODO: Would it be faster to handle a Despawn packet instead of looping through here?
        let mut server_players = HashSet::new();
        for player in update_players.players.iter() {
            server_players.insert(player.id);
            if let Some((p, _entity)) = players.get_mut(&player.id) {
                p.x = player.local_pos.0;
                p.y = player.local_pos.1;
                p.z = player.local_pos.2;
            } else {
                // New player
                spawn_player(&mut commands, &asset_server, player, player.id == client_id.id);
            }
        }
        
        // Remove despawned players
        for removed_player in player_ids.difference(&server_players) {
            let (pos, entity) = players.get(removed_player).unwrap();
            info!("[client] Despawning player with id={} at position={:?}", removed_player, pos);
            commands.entity(*entity).despawn();
        }
    }
}

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, player: &PlayerInfo, is_self: bool) {
    info!("[client] Spawning new player at {:?}", player.local_pos);
    // TODO: get player scale from server
    let mut player_spawn = commands
        .spawn(SceneBundle {
            scene: asset_server.load(&player.handle_id),
            transform: Transform::from_xyz(player.local_pos.0, player.local_pos.1, player.local_pos.2).with_scale(Vec3::splat(0.05)),
            ..default()
        });
    player_spawn
        .insert(ClientPlayer { id: player.id })
        .insert(Position { x: player.local_pos.0, y: player.local_pos.1, z: player.local_pos.2 });

    if is_self {
        player_spawn.insert(Me);
    }
}