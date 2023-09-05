use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Res, SceneBundle, Transform};

use crate::resource::LevelInfo;

pub fn load_level(commands: &mut Commands, asset_server: &Res<AssetServer>, level: &LevelInfo) {
    let mut transform =
        Transform::from_xyz(level.scene_transform[0], level.scene_transform[1], level.scene_transform[2]).with_scale(Vec3::splat(level.scale));
    transform.rotate_x(level.scene_transform[3]);
    commands.spawn(SceneBundle { scene: asset_server.load(&level.handle_id), transform, ..default() });
}
