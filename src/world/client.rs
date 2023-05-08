use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, IntoSystemConfig, NextState, OnUpdate, Plugin, Res, ResMut};
use crate::networking::server_packets::LevelInfo;
use crate::state::client::ClientState;
use crate::world;

pub struct WorldClientPlugin;
impl Plugin for WorldClientPlugin {

    fn build(&self, app: &mut App) {
        app.add_system(spawn_scene.in_set(OnUpdate(ClientState::SpawnScene)));
    }
}

fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>, level: Res<LevelInfo>, mut client_state: ResMut<NextState<ClientState>>) {
    world::load_level(&mut commands, &asset_server, &level);
    client_state.set(ClientState::Running);
}

// 
// fn handle_change_level(mut commands: Commands, mut manager: ResMut<ClientPacketManager>) {
//     if let Some(change_levels) = manager.received::<ChangeLevel, ChangeLevelPacketBuilder>(false).unwrap() {
//         commands.insert_resource(LevelSelection::Iid(change_levels.last().unwrap().level_iid.to_string()));
//     }
// }