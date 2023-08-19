use bevy::prelude::Bundle;
use bevy_rapier3d::prelude::*;

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
}
