use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

/// Level metadata
#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct LevelInfo {
    pub handle_id: String,
    // x, y, z, x-rotation
    pub scene_transform: [f32; 4],
    pub scale: f32,
}
