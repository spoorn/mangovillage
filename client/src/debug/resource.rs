use bevy::prelude::Resource;
use derivative::Derivative;

/// Mutable zoom speed
#[derive(Derivative, Resource)]
#[derivative(Default)]
pub struct ZoomSpeed {
    #[derivative(Default(value="5.0"))]
    pub speed: f32
}

/// Visibility of meshes
#[derive(Resource)]
pub struct MeshVisibility {
    pub visible: bool
}