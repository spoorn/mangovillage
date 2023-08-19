use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterControllerOutput;
use bevy_rapier3d::prelude::{
    CharacterLength, Collider, KinematicCharacterController, LockedAxes, QueryFilter, QueryFilterFlags, RapierContext, RigidBody, TOIStatus,
};

use mangovillage_common::networking::client_packets::{Movement, MovementPacketBuilder};
use mangovillage_common::networking::server_packets::{Player, Players};
use mangovillage_common::physics::component::ColliderBundle;
use mangovillage_common::player;
use mangovillage_common::player::component::PlayerData;
use mangovillage_common::player::PLAYER_MODEL_HANDLE_IDS;
use player::get_player_collider;

use crate::networking::resource::ServerPacketManager;
use crate::player::component::{ServerPlayer, ServerPlayerBundle};
use crate::state::ServerState;

pub mod component;

const PLAYER_MOVEMENT_SPEED: f32 = 1000.0;
const GRAVITY_STEP_SPEED: f32 = 100.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (broadcast_players, players_move, player_collision).run_if(in_state(ServerState::Running)));
        // Run collision handling in substep schedule
        //.add_systems(SubstepSchedule, player_collision.run_if(in_state(ServerState::Running)).in_set(SubstepSet::SolveUserConstraints));
    }
}

fn players_move(
    mut manager: ResMut<ServerPacketManager>,
    mut players: Query<(&ServerPlayer, &PlayerData, &mut KinematicCharacterController)>,
    time: Res<Time>,
) {
    let move_packets = manager.received_all::<Movement, MovementPacketBuilder>(false).unwrap();
    for (addr, move_packets) in move_packets {
        if let Some(move_packet) = move_packets {
            let movement = move_packet.last().unwrap();
            // Find player
            let mut found = false;
            for (server_player, player_data, mut controller) in players.iter_mut() {
                if server_player.addr == addr && player_data.id == manager.get_client_id(&addr).unwrap() {
                    found = true;
                    let movement_vec = Vec2::new(movement.translation[0], movement.translation[1]).normalize();
                    let dx = movement_vec.x * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
                    let dy = movement_vec.y * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
                    match controller.translation {
                        None => controller.translation = Some(Vec3::new(dx, dy, 0.0)),
                        Some(mut translation) => {
                            translation.x += dx;
                            translation.y += dy;
                        }
                    };
                    break;
                }
            }
            if !found {
                error!("Received move packet from invalid player.  Packet from addr={}, id={}", addr, manager.get_client_id(&addr).unwrap());
            }
        }
    }
}

fn player_collision(
    rapier_context: Res<RapierContext>,
    mut players: Query<
        (&Transform, &Collider, &mut KinematicCharacterController, Option<&mut KinematicCharacterControllerOutput>),
        With<ServerPlayer>,
    >,
    time: Res<Time>,
) {
    for (transform, collider, mut controller, controller_output) in players.iter_mut() {
        //println!("output: {:?}", controller_output);
        // Shape cast to check if we are grounded
        // We do this manually instead of `output.grounded` so the grounded check is always consistent
        if rapier_context.cast_shape(transform.translation, transform.rotation, Vec3::NEG_Z, collider, 0.11, QueryFilter::only_fixed()).is_none() {
            match controller.translation {
                None => controller.translation = Some(Vec3::NEG_Z * GRAVITY_STEP_SPEED * time.delta_seconds()),
                Some(mut translation) => translation.z -= GRAVITY_STEP_SPEED * time.delta_seconds(),
            }
        } else {
            if let Some(output) = controller_output {
                // Keep player above ground
                for collision in &output.collisions {
                    //println!("collision: {:?}", collision);
                    // TODO: handle other statuses
                    if collision.toi.status == TOIStatus::Converged {
                        let penetration = collision.translation_remaining.length();
                        let angle = collision.toi.normal1.angle_between(Vec3::Z);
                        let dz = (collision.toi.normal1.z * penetration).abs();
                        match controller.translation {
                            None => controller.translation = Some(Vec3::new(0.0, 0.0, dz)),
                            Some(mut translation) => {
                                if translation.z <= 0.0 {
                                    translation.z = dz;
                                } else {
                                    translation.z += dz
                                }
                            }
                        }
                    }
                }
            }
        }
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
    let mut transform = Transform::from_xyz(-10.0, 0.0, 150.0).with_scale(Vec3::splat(1.0));
    transform.rotate_x(0.0);
    let player_model = PLAYER_MODEL_HANDLE_IDS[player_data.handle_id as usize];
    let entity = commands.spawn(SceneBundle { scene: asset_server.load(player_model), transform, ..default() }).id();
    debug!("Player EntityId={:?}", entity);
    commands
        .entity(entity)
        .insert(ServerPlayerBundle {
            server_player: ServerPlayer { addr },
            player_data,
            colliders: ColliderBundle {
                collider: get_player_collider(),
                rigid_body: RigidBody::KinematicPositionBased,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..default()
            },
        })
        .insert(KinematicCharacterController {
            up: Vec3::Z,
            offset: CharacterLength::Absolute(0.1),
            slide: false,
            //autostep: None,
            // autostep: Some(CharacterAutostep {
            //     max_height: CharacterLength::Absolute(200.0),
            //     min_width: CharacterLength::Absolute(200.0),
            //     include_dynamic_bodies: false,
            // }),
            max_slope_climb_angle: 85.0,
            min_slope_slide_angle: 0.0,
            apply_impulse_to_dynamic_bodies: false,
            snap_to_ground: Some(CharacterLength::Absolute(10000.0)),
            filter_flags: QueryFilterFlags::ONLY_FIXED,
            ..default()
        });
}
