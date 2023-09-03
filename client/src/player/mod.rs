use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use mangovillage_common::networking::client_packets::Movement;
use mangovillage_common::networking::server_packets::Player;
use mangovillage_common::networking::server_packets::{Players, PlayersPacketBuilder};
use mangovillage_common::player;
use mangovillage_common::player::component::PlayerData;
use mangovillage_common::player::{set_player_rotation, PLAYER_MODEL_HANDLE_IDS};
use player::get_player_collider;

use crate::networking::resource::ClientPacketManager;
use crate::player::component::Me;
use crate::player::resource::ClientId;
use crate::state::ClientState;

pub mod component;
pub mod resource;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_players, movement).run_if(in_state(ClientState::Running)));
    }
}

// TODO: optimize networking
fn movement(mut manager: ResMut<ClientPacketManager>, mouse_button_input: Res<Input<MouseButton>>, windows: Query<&Window, With<PrimaryWindow>>) {
    if mouse_button_input.pressed(MouseButton::Right) {
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
    client_id: Res<ClientId>,
    meshes: Query<(Entity, &Handle<Mesh>), Without<NoFrustumCulling>>,
    time: Res<Time>,
) {
    // for (entity, client_player_data, mut transform) in players_query.iter_mut() {
    //     println!("position {:}", transform.translation);
    // }
    // TODO: properly disable frustum culling for player meshes only due to bug https://github.com/bevyengine/bevy/issues/4294
    for (entity, _mesh) in &meshes {
        commands.entity(entity).insert(NoFrustumCulling);
    }
    let server_player_packets = manager.received::<Players, PlayersPacketBuilder>(false).unwrap();
    if let Some(mut server_players) = server_player_packets {
        // Only care about last packet
        let server_players = server_players.swap_remove(server_players.len() - 1);
        // Find differences and intersections
        let mut server_players_map: HashMap<u32, Player> = server_players.players.into_iter().map(|player| (player.id, player)).collect();

        for (entity, client_player_data, mut transform) in players_query.iter_mut() {
            if let Some(server_player_info) = server_players_map.remove(&client_player_data.id) {
                // TODO: handle model changes
                // TODO: optimize
                let old_translation = transform.translation;
                transform.translation.x = server_player_info.transform[0];
                transform.translation.y = server_player_info.transform[1];
                transform.translation.z = server_player_info.transform[2];
                //println!("trans {:}", transform.translation);
                // transform.translation = old_translation;
                // if time.elapsed_seconds() as u32 / 2 % 2 == 0 {
                //     transform.translation.x += 0.5;
                // }
                let look_direction = Vec2::new(transform.translation.x - old_translation.x, transform.translation.y - old_translation.y);
                set_player_rotation(look_direction, &mut transform);
                transform.scale = Vec3::splat(server_player_info.scale);
                //println!("### transform {:?}", transform);
            } else {
                debug!("Removing player {}", client_player_data.id);
                commands.entity(entity).despawn_recursive();
            }
        }

        // TODO: handle spawning players in a separate system to optimize
        // New players
        server_players_map.into_iter().for_each(|(id, player)| {
            debug!("Adding new player {}", id);
            let mut transform =
                Transform::from_xyz(player.transform[0], player.transform[1], player.transform[2]).with_scale(Vec3::splat(player.scale));
            transform.look_to(Vec3::NEG_Y, Vec3::Z);
            let player_model = PLAYER_MODEL_HANDLE_IDS[player.handle_id as usize];
            let mut entity_comments = commands.spawn(SceneBundle { scene: asset_server.load(player_model), transform, ..default() });
            entity_comments
                .insert(PlayerData { id, handle_id: player.handle_id })
                // Add collider for debug rendering
                .insert(get_player_collider());

            if client_id.0 == id {
                entity_comments.insert(Me);
            }
        });
    }
}
