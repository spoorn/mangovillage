use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use mangovillage_common::networking::client_packets::{Movement, MovementPacketBuilder};
use std::time::Duration;

use mangovillage_common::networking::server_packets::{Player, Players};
use mangovillage_common::physics::component::ColliderBundle;
use mangovillage_common::player::component::PlayerData;
use mangovillage_common::player::PLAYER_MODEL_HANDLE_IDS;

use crate::networking::resource::ServerPacketManager;
use crate::player::component::{ServerPlayer, ServerPlayerBundle};
use crate::state::ServerState;

pub mod component;

const PLAYER_MOVEMENT_SPEED: f32 = 100.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (players_move, broadcast_players).run_if(in_state(ServerState::Running)));
    }
}

fn players_move(
    mut manager: ResMut<ServerPacketManager>,
    mut controllers: Query<(Entity, &ServerPlayer, &PlayerData, &mut KinematicCharacterController)>,
    mut character_controller_outputs: Query<&mut KinematicCharacterControllerOutput>,
    mut global_transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    let move_packets = manager.received_all::<Movement, MovementPacketBuilder>(false).unwrap();
    for (addr, move_packets) in move_packets {
        if let Some(move_packet) = move_packets {
            let movement = move_packet.last().unwrap();
            // Find player
            let mut found = false;
            for (_entity, server_player, player_data, mut controller) in controllers.iter_mut() {
                if server_player.addr == addr && player_data.id == manager.get_client_id(&addr).unwrap() {
                    found = true;
                    controller.translation = Some(
                        Vec3::new(movement.translation[0], movement.translation[1], -1.0).normalize() * PLAYER_MOVEMENT_SPEED * time.delta_seconds(),
                    );
                    break;
                }
            }
            if !found {
                error!("Received move packet from invalid player.  Packet from addr={}, id={}", addr, manager.get_client_id(&addr).unwrap());
            }
        }
    }

    for (entity, _server_player, _player_data, mut controller) in controllers.iter_mut() {
        println!("### controller {:?}", controller.translation);
        match character_controller_outputs.get(entity) {
            Ok(output) => {
                println!("### grounded {:?}", output);
                // Sum vertical collisions to avoid penetrating colliders, but ignore anything outside of Z direction as
                // we only want player's controls for horizontal movement
                for collision in &output.collisions {
                    if controller.translation.is_none() {
                        controller.translation = Some(Vec3::ZERO);
                    }

                    let character_global_witness = global_transforms.get(entity).unwrap().translation() + collision.toi.witness2;
                    let collider_global_witness = global_transforms.get(collision.entity).unwrap().translation() + collision.toi.witness1;
                    println!("### character grounded pos {:?}", global_transforms.get(entity).unwrap().translation());
                    println!("### character_global_witness {:?}", character_global_witness);
                    println!("### collider_global_witness {:?}", collider_global_witness);
                    let mut flipped_normal = (character_global_witness - collider_global_witness);
                    println!("### flipped_normal {:?}", flipped_normal);
                    // If within offset
                    if flipped_normal.length() + f32::EPSILON < 1.0 {
                        // Bump up
                        flipped_normal =
                            flipped_normal.clamp_length_max(1.0 - flipped_normal.length() - 0.1) * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
                        let angle = flipped_normal.angle_between(Vec3::Z);

                        println!("### updated flipped_normal {:?}", flipped_normal);
                        //controller.translation.as_mut().unwrap().z = flipped_normal.length() * angle.cos();
                        //if angle < 85.0 {
                        // Handle horizontal collisions
                        // controller.translation.as_mut().unwrap().x += flipped_normal.x;
                        // controller.translation.as_mut().unwrap().y += flipped_normal.y;
                        // controller.translation.as_mut().unwrap().z += flipped_normal.z;
                        //}
                    }
                }

                // Also apply gravity if not grounded
                if !output.grounded {
                    println!("### character NOT grounded pos {:?}", global_transforms.get(entity).unwrap().translation());
                    match controller.translation {
                        None => controller.translation = Some(Vec3::new(0.0, 0.0, -10.0)),
                        Some(mut translation) => translation.z = -10.0,
                    }
                }
                println!("### translation {:?}", controller.translation);
            }
            Err(_) => {
                // No output yet, default to gravity only
                match controller.translation {
                    None => controller.translation = Some(Vec3::new(0.0, 0.0, -10.0)),
                    Some(mut translation) => translation.z = -10.0,
                }
            }
        }
    }
    //std::thread::sleep(Duration::from_secs(1));
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
    let mut transform = Transform::from_xyz(-10.0, 0.0, 100.0).with_scale(Vec3::splat(0.05));
    transform.rotate_x(0.0);
    let player_model = PLAYER_MODEL_HANDLE_IDS[player_data.handle_id as usize];
    commands
        .spawn(SceneBundle { scene: asset_server.load(player_model), transform, ..default() })
        .insert(ServerPlayerBundle {
            server_player: ServerPlayer { addr },
            player_data,
            colliders: ColliderBundle {
                collider: Collider::capsule_z(5.0, 12.0),
                rigid_body: RigidBody::KinematicPositionBased,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                //ccd: Ccd::enabled(),
                ..default()
            },
        })
        .insert(KinematicCharacterController {
            //translation: Some(Vec3::new(0.0, 0.0, -10.0)),
            up: Vec3::Z,
            slide: false,
            offset: CharacterLength::Absolute(1.0),
            autostep: None,
            // autostep: Some(CharacterAutostep {
            //     max_height: CharacterLength::Relative(200.0),
            //     min_width: CharacterLength::Relative(200.0),
            //     include_dynamic_bodies: true,
            // }),
            //snap_to_ground: Some(CharacterLength::Absolute(100000.0)),
            max_slope_climb_angle: 85.0_f32.to_radians(),
            min_slope_slide_angle: 0.0,
            apply_impulse_to_dynamic_bodies: false,
            ..default()
        })
        .insert(GravityScale(0.0));
}
