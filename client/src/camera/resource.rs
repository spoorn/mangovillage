use bevy::prelude::{Resource, Transform};
use derivative::Derivative;

/// Locked camera state
#[derive(Derivative, Resource)]
#[derivative(Default)]
pub struct LockedCameraState {
    pub transform: Transform,
}

/// Debug camera state
#[derive(Resource)]
pub struct DebugCameraState {
    pub transform: Transform,
    pub camera_speed: CameraSpeed,
}

/// Mutable zoom speed
#[derive(Derivative)]
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
