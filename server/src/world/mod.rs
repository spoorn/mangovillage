use bevy::prelude::*;
use mangovillage_common::resource::LevelInfo;
use mangovillage_common::world;
use crate::state::ServerState;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_world.run_if(in_state(ServerState::LoadWorld)));
    }
}

/// Loads world into server
fn load_world(mut commands: Commands, asset_server: Res<AssetServer>, mut server_state: ResMut<NextState<ServerState>>) {
    let level = LevelInfo {
        handle_id: "models/volcano_island_lowpoly/scene.gltf#Scene0".to_string(),
        scene_transform: [0.0, 0.0, 0.0, std::f32::consts::PI / 2.0],
        scale: 0.005
    };
    info!("[server] Spawning level {}", level.handle_id);
    world::load_level(&mut commands, &asset_server, &level);
    server_state.set(ServerState::Running);
}