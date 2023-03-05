use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Plugin, Res};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSpawnBehavior, SetClearColor};

pub struct LdtkClientPlugin;
impl Plugin for LdtkClientPlugin {

    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_startup_system(load_level);
    }
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/test.ldtk"),
        ..default()
    });
}