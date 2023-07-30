use bevy::prelude::*;
use mangovillage_common::resource::LevelInfo;
use crate::state::ClientState;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_scene.run_if(in_state(ClientState::SpawnScene)));
    }
}

fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>, mut client_state: ResMut<NextState<ClientState>>) {
    load_level(&mut commands, &asset_server, &LevelInfo {
        handle_id: "models/volcano_island_lowpoly/scene.gltf#Scene0".to_string(),
        scene_transform: [0.0, 0.0, 0.0, std::f32::consts::PI / 2.0],
        scale: 0.005
    });
    client_state.set(ClientState::Running);
}

fn load_level(commands: &mut Commands, asset_server: &Res<AssetServer>, level: &LevelInfo) {
    info!("[client] Spawning level {}", level.handle_id);
    let mut scene_transform = Transform::from_xyz(level.scene_transform[0], level.scene_transform[1], level.scene_transform[2]).with_scale(Vec3::splat(level.scale));
    scene_transform.rotate_x(level.scene_transform[3]);
    commands.spawn(SceneBundle {
        scene: asset_server.load(&level.handle_id),
        transform: scene_transform,
        ..default()
    });
}