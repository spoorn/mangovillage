use bevy::prelude::Resource;
use derivative::Derivative;

/// Mutable zoom speed
#[derive(Derivative, Resource)]
#[derivative(Default)]
pub struct CameraSpeed {
    #[derivative(Default(value = "50.0"))]
    pub zoom_speed: f32,
    #[derivative(Default(value = "0.005"))]
    pub move_speed: f32,
}

/// Visibility of meshes
#[derive(Resource)]
pub struct MeshVisibility {
    pub visible: bool,
}
