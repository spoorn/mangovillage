use bevy::prelude::{Commands, default, Res, SceneBundle, Transform};
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use crate::resource::LevelInfo;

pub fn load_level(commands: &mut Commands, asset_server: &Res<AssetServer>, level: &LevelInfo) {
    let mut scene_transform = Transform::from_xyz(level.scene_transform[0], level.scene_transform[1], level.scene_transform[2]).with_scale(Vec3::splat(level.scale));
    scene_transform.rotate_x(level.scene_transform[3]);
    commands.spawn(SceneBundle {
        scene: asset_server.load(&level.handle_id),
        transform: scene_transform,
        ..default()
    });
}
