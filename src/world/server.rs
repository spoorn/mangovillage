use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Plugin, Res};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSet, LevelSpawnBehavior};
use crate::world::LEVEL_IIDS;

pub struct LdtkServerPlugin;
impl Plugin for LdtkServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_startup_system(load_level);
    }
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/test.ldtk"),
        level_set: LevelSet { iids: LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect() },
        ..default()
    });
}

