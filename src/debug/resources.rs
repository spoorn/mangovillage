use bevy::prelude::Resource;
use derivative::Derivative;

// Mutable zoom speed
#[derive(Derivative, Resource)]
#[derivative(Default)]
pub struct ZoomSpeed {
    #[derivative(Default(value="0.5"))]
    pub speed: f32
}

#[derive(Resource)]
pub struct MeshVisibility {
    pub visible: bool
}