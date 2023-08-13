use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use mangovillage_common::networking::server_packets::{Player, Players};
use mangovillage_common::physics::component::ColliderBundle;
use mangovillage_common::player::component::PlayerData;
use mangovillage_common::player::PLAYER_MODEL_HANDLE_IDS;

use crate::networking::resource::ServerPacketManager;
use crate::player::component::{ServerPlayer, ServerPlayerBundle};
use crate::state::ServerState;

pub mod component;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (players_move, broadcast_players).run_if(in_state(ServerState::Running)));
    }
}

fn players_move(mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(0.0, 0.0, -1.0));
    }
}

fn broadcast_players(mut manager: ResMut<ServerPacketManager>, player_query: Query<(&PlayerData, &Transform)>) {
    // TODO: optimize
    // TODO: make Copy instead of Cloned
    let players = player_query
        .iter()
        .map(|(player_data, transform)| Player {
            id: player_data.id,
            handle_id: player_data.handle_id,
            transform: [transform.translation.x, transform.translation.y, transform.translation.z, transform.rotation.x],
            scale: transform.scale.x,
        })
        .collect();
    manager.broadcast(Players { players }).unwrap();
}

pub fn spawn_player(commands: &mut Commands, addr: String, id: u32, asset_server: &Res<AssetServer>) {
    info!("[server] Spawning player with addr={}, id={}", addr, id);
    let player_data = PlayerData { id, handle_id: 0 };
    let mut transform = Transform::from_xyz(-10.0, 0.0, 50.0).with_scale(Vec3::splat(0.05));
    transform.rotate_x(0.0);
    let player_model = PLAYER_MODEL_HANDLE_IDS[player_data.handle_id as usize];
    commands
        .spawn(SceneBundle { scene: asset_server.load(player_model), transform, ..default() })
        .insert(ServerPlayerBundle {
            server_player: ServerPlayer { addr },
            player_data,
            colliders: ColliderBundle {
                collider: Collider::cuboid(12.0, 12.0, 12.0),
                rigid_body: RigidBody::KinematicPositionBased,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..default()
            },
        })
        .insert(KinematicCharacterController {
            translation: Some(Vec3::new(0.0, 0.0, -1.0)),
            up: Vec3::Z,
            slide: false,
            max_slope_climb_angle: 45.0_f32.to_radians(),
            min_slope_slide_angle: 0.0,
            apply_impulse_to_dynamic_bodies: false,
            ..default()
        });
}
