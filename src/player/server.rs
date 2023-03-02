use bevy::app::App;
use bevy::prelude::{Plugin, Query, ResMut};
use bevy::utils::HashMap;
use crate::common::components::Position;
use crate::networking::client_packets::{Move, MovePacketBuilder};
use crate::networking::server_packets::{PlayerPosition, UpdatePlayerPositions};
use crate::player::components::Player;
use crate::player::handle_move;
use crate::server::resources::ServerPacketManager;

pub struct PlayerServerPlugin;
impl Plugin for PlayerServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_system(send_player_positions)
             .add_system(handle_player_move);
    }
}

// TODO: optimize: Don't send all player positions constantly, only changed
fn send_player_positions(players: Query<(&Player, &Position)>, mut manager: ResMut<ServerPacketManager>) {
    // TODO: send players per map to clients in that map
    let mut pps: Vec<PlayerPosition> = Vec::new();
    for (player, pos) in players.iter() {
        pps.push(PlayerPosition {
            id: player.id,
            position: (pos.x, pos.y)
        });
    }
    
    manager.broadcast(UpdatePlayerPositions { positions: pps }).unwrap();
}

fn handle_player_move(mut players_query: Query<(&Player, &mut Position)>, mut manager: ResMut<ServerPacketManager>) {
    let move_packets = manager.received_all::<Move, MovePacketBuilder>(false).unwrap();
    
    if !move_packets.is_empty() {
        // TODO: there has to be a faster way to do this than creating a map every iteration?  Can use a set too
        let mut players = HashMap::new();
        for (player, position) in players_query.iter_mut() {
            players.insert(player.id, position);
        }
        
        for (addr, moves) in move_packets.iter() {
            if let Some(moves) = moves {
                // We only care about the last movement from the player
                //for last in moves.iter() {
                if let Some(last) = moves.last() {
                    let player_id = manager.get_client_id(addr).unwrap();
                    if let Some(mut position) = players.get_mut(&player_id) {
                        handle_move(last.dir, &mut position);
                    }
                }
            }
        }
    }
}