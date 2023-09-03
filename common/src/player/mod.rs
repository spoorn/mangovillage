use bevy::math::{Vec2, Vec3};
use bevy::prelude::Transform;
use bevy_rapier3d::prelude::Collider;

pub mod component;

pub static PLAYER_MODEL_HANDLE_IDS: [&str; 2] = ["models/amber/Amber.glb#Scene0", "models/owl/scene.gltf#Scene0"];

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
