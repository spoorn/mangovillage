//! Common components that can be used across multiple systems.

use bevy::prelude::Component;

#[derive(Component)]
pub struct MoveTarget {
    /// (x, y)
    pub target: (f32, f32),
}
