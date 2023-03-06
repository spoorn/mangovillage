use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Plugin, Res, ResMut};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection, LevelSpawnBehavior, SetClearColor};
use crate::client::resources::ClientPacketManager;
use crate::networking::server_packets::{ChangeLevel, ChangeLevelPacketBuilder};

pub struct LdtkClientPlugin;
impl Plugin for LdtkClientPlugin {

    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_startup_system(load_level)
            .add_system(handle_change_level);
    }
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/test.ldtk"),
        ..default()
    });
}

fn handle_change_level(mut commands: Commands, mut manager: ResMut<ClientPacketManager>) {
    if let Some(change_levels) = manager.received::<ChangeLevel, ChangeLevelPacketBuilder>(false).unwrap() {
        commands.insert_resource(LevelSelection::Iid(change_levels.last().unwrap().level_iid.to_string()));
    }
}