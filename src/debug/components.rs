use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct MouseCoordinateText;

/// Tags an entity as capable of panning and orbiting.
/// 
/// From https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
#[derive(Default, Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}