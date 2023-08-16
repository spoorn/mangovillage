use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_xpbd_3d::prelude::*;

use mangovillage_common::networking::client_packets::Movement;
use mangovillage_common::networking::server_packets::Player;
use mangovillage_common::networking::server_packets::{Players, PlayersPacketBuilder};
use mangovillage_common::player::component::PlayerData;
use mangovillage_common::player::PLAYER_MODEL_HANDLE_IDS;

use crate::networking::resource::ClientPacketManager;
use crate::state::ClientState;

pub mod resource;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_players, movement).run_if(in_state(ClientState::Running)));
    }
}

// TODO: optimize networking
fn movement(
    mut manager: ResMut<ClientPacketManager>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(mut position) = window.cursor_position() {
            // Get position with origin at center of window
            // y is flipped
            position.x -= window.width() / 2.0;
            position.y = window.height() / 2.0 - position.y;
            manager.send(Movement { translation: position.to_array() }).unwrap();
        }
    }
}

fn update_players(
    mut manager: ResMut<ClientPacketManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut players_query: Query<(Entity, &PlayerData, &mut Transform)>,
) {
    let server_player_packets = manager.received::<Players, PlayersPacketBuilder>(false).unwrap();
    if let Some(mut server_players) = server_player_packets {
        // Only care about last packet
        let server_players = server_players.swap_remove(server_players.len() - 1);
        // Find differences and intersections
        let mut server_players_map: HashMap<u32, Player> =
            server_players.players.into_iter().map(|player| (player.id, player)).collect();

        for (entity, client_player_data, mut transform) in players_query.iter_mut() {
            if let Some(server_player_info) = server_players_map.remove(&client_player_data.id) {
                // TODO: handle model changes
                // TODO: optimize
                transform.translation.x = server_player_info.transform[0];
                transform.translation.y = server_player_info.transform[1];
                transform.translation.z = server_player_info.transform[2];
                transform.rotation.x = server_player_info.transform[3];
                transform.scale = Vec3::splat(server_player_info.scale);
            } else {
                debug!("Removing player {}", client_player_data.id);
                commands.entity(entity).despawn_recursive();
            }
        }

        // New players
        server_players_map.into_iter().for_each(|(id, player)| {
            debug!("Adding new player {}", id);
            let mut transform = Transform::from_xyz(player.transform[0], player.transform[1], player.transform[2])
                .with_scale(Vec3::splat(player.scale));
            transform.rotate_x(player.transform[3]);
            let player_model = PLAYER_MODEL_HANDLE_IDS[player.handle_id as usize];
            commands
                .spawn(SceneBundle { scene: asset_server.load(player_model), transform, ..default() })
                .insert(PlayerData { id, handle_id: player.handle_id })
                // Add collider for debug rendering
                .insert(Collider::capsule(10.0, 12.0));
        });
    }
}
