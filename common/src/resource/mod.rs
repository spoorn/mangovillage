use bevy::prelude::Resource;

/// Level metadata
#[derive(Resource)]
pub struct LevelInfo {
    pub handle_id: String,
    // x, y, z, x-rotation
    pub scene_transform: [f32; 4],
    pub scale: f32
}