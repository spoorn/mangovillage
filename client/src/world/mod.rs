use bevy::prelude::*;

use mangovillage_common::networking::server_packets::{SpawnScene, SpawnScenePacketBuilder};
use mangovillage_common::world;

use crate::networking::resource::ClientPacketManager;
use crate::state::ClientState;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_scene.run_if(in_state(ClientState::Running)));
    }
}

fn spawn_scene(
    mut manager: ResMut<ClientPacketManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client_state: ResMut<NextState<ClientState>>,
) {
    let spawn_scene_packets = manager.received_all::<SpawnScene, SpawnScenePacketBuilder>(false).unwrap();
    for (_addr, spawn_scenes) in spawn_scene_packets {
        if let Some(spawn_scenes) = spawn_scenes {
            let scene = spawn_scenes.last().unwrap();
            info!("[client] Spawning level {}", scene.level.handle_id);
            world::load_level(&mut commands, &asset_server, &scene.level);
            client_state.set(ClientState::LoadingLevel);
        }
    }
}
