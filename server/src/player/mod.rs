use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterControllerOutput;
use bevy_rapier3d::prelude::{
    CharacterLength, Collider, KinematicCharacterController, LockedAxes, QueryFilter, QueryFilterFlags, RapierContext, RigidBody, TOIStatus,
};

use mangovillage_common::component::MoveTarget;
use mangovillage_common::networking::client_packets::{Movement, MovementPacketBuilder};
use mangovillage_common::networking::server_packets::{Player, Players};
use mangovillage_common::physics::component::ColliderBundle;
use mangovillage_common::player;
use mangovillage_common::player::component::PlayerData;
use mangovillage_common::player::{set_player_rotation, PLAYER_MODEL_HANDLE_IDS};
use player::get_player_collider;

use crate::networking::resource::ServerPacketManager;
use crate::player::component::{ServerPlayer, ServerPlayerBundle};
use crate::state::ServerState;

pub mod component;

const PLAYER_MOVEMENT_SPEED: f32 = 100.0;
const GRAVITY_STEP_SPEED: f32 = 100.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (broadcast_players, players_move, movement, player_collision).run_if(in_state(ServerState::Running)));
        // Run collision handling in substep schedule
        //.add_systems(SubstepSchedule, player_collision.run_if(in_state(ServerState::Running)).in_set(SubstepSet::SolveUserConstraints));
    }
}

fn movement(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Transform, &mut MoveTarget, &mut KinematicCharacterController), With<ServerPlayer>>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut move_target, mut controller) in players.iter_mut() {
        let movement_vec = Vec2::new(move_target.target.0, move_target.target.1).normalize();
        let dx = movement_vec.x * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
        let dy = movement_vec.y * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
        match controller.translation {
            None => controller.translation = Some(Vec3::new(dx, dy, 0.0)),
            Some(ref mut translation) => {
                translation.x += dx;
                translation.y += dy;
            }
        };
        let translation = controller.translation.unwrap();
        //println!("trans: {:?}", transform.translation);
        set_player_rotation(translation.xy(), &mut transform);
        move_target.target.0 -= dx;
        move_target.target.1 -= dy;
        if move_target.target.0.abs() < 0.05 && move_target.target.1.abs() < 0.05 {
            commands.entity(entity).remove::<MoveTarget>();
        }
        //println!("move: {:?}", controller.translation);
    }
}

fn players_move(mut manager: ResMut<ServerPacketManager>, mut commands: Commands, mut players: Query<(Entity, &ServerPlayer, &PlayerData)>) {
    let move_packets = manager.received_all::<Movement, MovementPacketBuilder>(false).unwrap();
    for (addr, move_packets) in move_packets {
        if let Some(move_packet) = move_packets {
            let movement = move_packet.last().unwrap();
            // Find player
            let mut found = false;
            for (entity, server_player, player_data) in players.iter_mut() {
                if server_player.addr == addr && player_data.id == manager.get_client_id(&addr).unwrap() {
                    found = true;
                    // TODO: path to spot in world?
                    let movement_vec = Vec2::new(movement.translation[0], movement.translation[1]).normalize() * 0.2;
                    commands.entity(entity).insert(MoveTarget { target: (movement_vec.x, movement_vec.y) });
                    break;
                }
            }
            if !found {
                error!("Received move packet from invalid player.  Packet from addr={}, id={}", addr, manager.get_client_id(&addr).unwrap());
            }
        }
    }
}

/// Player collision system.  Keeps the player afloat colliders, but don't apply horizontal forces from collisions.
///
/// Automatically does not run in parallel with the player movement system since they access the same data mutably.
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
        if rapier_context.cast_shape(transform.translation, transform.rotation, Vec3::NEG_Z, collider, 1.9, QueryFilter::only_fixed()).is_none() {
            match controller.translation {
                None => controller.translation = Some(Vec3::NEG_Z * GRAVITY_STEP_SPEED * time.delta_seconds()),
                Some(mut translation) => translation.z -= GRAVITY_STEP_SPEED * time.delta_seconds(),
            }
            //println!("gravity {:?}", controller.translation);
        } else {
            // TODO: handle oscillating collisions, such as bouncing up and down between above and below colliders indefinitely
            if let Some(output) = controller_output {
                //println!("collisions: {:?}", output);
                // Keep player above ground
                for collision in &output.collisions {
                    //println!("collision: {:?}", collision);
                    // TODO: handle other statuses
                    if collision.toi.status == TOIStatus::Converged {
                        let penetration = collision.translation_remaining.length();
                        let angle = collision.toi.normal1.angle_between(Vec3::Z);
                        // Since we only allow player controlled movement to be horizontal, penetration at this point is
                        // also always horizontal.  We want to find the vertical distance to pop up from the end of the
                        // penetration, assuming the normal surface is flat, it will pop us out of the current collision.
                        // let p = penetration length
                        // let h = vertical distance we want to pop out of
                        // let theta = angle from normal vector to +Z
                        // based on triangle symmetry, we can get
                        //      h = p * tan(theta)
                        // However, if we are at a low enough angle -> 0, tan function will start to decrease and at 0
                        // this function will just give us 0 which is incorrect.  If there is penetration when
                        // theta == 0, we are penetrating vertically, which shouldn't happen, but if it does, just keep
                        //      h = p
                        // to pop up the full penetration length.
                        let mut dz = penetration;
                        if angle > 10.0 {
                            dz *= angle.tan()
                        }
                        // Add a small offset buffer so player floats slightly above ground
                        //dz += 0.2;
                        // divide by number of collisions to average it out
                        // TODO: optimize and make this properly check toi status
                        dz /= output.collisions.len() as f32;
                        // If collided entity is above, push it down
                        if collision.toi.normal1.z < 0.0 {
                            dz = -dz;
                        }
                        match controller.translation {
                            None => controller.translation = Some(Vec3::new(0.0, 0.0, dz)),
                            Some(ref mut translation) => {
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
        //println!("after collision: {:?}", controller.translation);
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
            transform: [transform.translation.x, transform.translation.y, transform.translation.z],
            scale: transform.scale.x,
        })
        .collect();
    manager.broadcast(Players { players }).unwrap();
}

pub fn spawn_player(commands: &mut Commands, addr: String, id: u32, asset_server: &Res<AssetServer>) {
    info!("[server] Spawning player with addr={}, id={}", addr, id);
    let player_data = PlayerData { id, handle_id: 0 };
    let mut transform = Transform::from_xyz(-10.0, 0.0, 150.0).with_scale(Vec3::splat(1.0));
    transform.look_to(Vec3::NEG_Y, Vec3::Z);
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
        // Our collision system manually handles a lot of what the character controller gives us, so we have more custom
        // tuning and control over parameters and behavior.  The defaults don't work so well with dramatic terrains.
        // This is mainly here so we can collect and act on collisions.
        .insert(KinematicCharacterController {
            up: Vec3::Z,
            // Use a low value because our collision system above already factors in an offset
            offset: CharacterLength::Absolute(2.0),
            slide: false,
            // Our collision system uses normal forces to automatically do autostep on slopes.
            // We can add this back or modify the collision system to add step heights if needed
            autostep: None,
            // autostep: Some(CharacterAutostep {
            //     max_height: CharacterLength::Absolute(200.0),
            //     min_width: CharacterLength::Absolute(200.0),
            //     include_dynamic_bodies: false,
            // }),
            max_slope_climb_angle: 85.0,
            min_slope_slide_angle: 0.0,
            apply_impulse_to_dynamic_bodies: false,
            // Our collision system uses gravity to push to ground already
            snap_to_ground: None, //Some(CharacterLength::Absolute(10000.0)),
            filter_flags: QueryFilterFlags::ONLY_FIXED,
            ..default()
        });
}
