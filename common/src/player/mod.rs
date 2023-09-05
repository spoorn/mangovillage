use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, AssetServer, Commands, Res, SceneBundle, Transform};
use bevy_rapier3d::prelude::Collider;

pub mod component;

pub static PLAYER_MODEL_HANDLE_IDS: [&str; 2] = ["models/amber/Amber.glb", "models/owl/scene.gltf"];

/// Common spawn player components between client and server
pub fn spawn_player<'a, 'w, 's>(
    commands: &'a mut Commands<'w, 's>,
    transform: Transform,
    handle_id: u8,
    asset_server: &Res<AssetServer>,
) -> EntityCommands<'w, 's, 'a> {
    let mut player_model = String::new();
    player_model.push_str(PLAYER_MODEL_HANDLE_IDS[handle_id as usize]);
    player_model.push_str("#Scene0");
    commands.spawn(SceneBundle { scene: asset_server.load(player_model), transform, ..default() })
}

/// Get's the default player collider
///
/// Due to some issue with rapier not reading Transforms, this has to follow the model's initial state and direction.
pub fn get_player_collider() -> Collider {
    Collider::capsule_y(1.0, 1.0)
}

/// Sets the player's facing direction
pub fn set_player_rotation(direction: Vec2, transform: &mut Transform) {
    if direction != Vec2::ZERO {
        transform.look_to(direction.extend(0.0), Vec3::Z);
    }
}
