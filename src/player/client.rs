use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, error, Input, KeyCode, Plugin, Query, Res, ResMut};
use bevy::utils::HashMap;

use crate::client::resources::ClientPacketManager;
use crate::common::components::Position;
use crate::common::Direction;
use crate::networking::client_packets::Move;
use crate::networking::server_packets::{UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder};
use crate::player::components::Player;
use crate::player::spawn_player;

pub struct PlayerClientPlugin;
impl Plugin for PlayerClientPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_system(movement_input)
            .add_system(update_players);
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

fn update_players(mut commands: Commands, mut players_query: Query<(&Player, &mut Position)>, mut manager: ResMut<ClientPacketManager>, asset_server: Res<AssetServer>) {
    //info!("manager {:?}", manager.manager);
    let update_players = manager.received::<UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder>(false).unwrap();
    if let Some(update_players) = update_players {
        // We only care about the last update
        if let Some(last) = update_players.last() {
            // TODO: there has to be a faster way to do this than creating a map every iteration?  Can use a set too
            let mut players = HashMap::new();
            for (player, position) in players_query.iter_mut() {
                players.insert(player.id, position);
            }
            
            for player in last.positions.iter() {
                if let Some(p) = players.get_mut(&player.id) {
                    p.x = player.position.0;
                    p.y = player.position.1;
                } else {
                    // New player
                    spawn_player(&mut commands, Some(&asset_server), player.id, player.position);
                }
            }
        }
    }
    
    // TODO: handle removed players, can probably optimize above a bit if we do this
}