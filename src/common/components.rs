use std::fmt::{Display, Formatter};
use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

/// Transform is world coordinates, LocalPosition is local coordinates on the map an entity is in
#[derive(Component, Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub damping: Damping,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}