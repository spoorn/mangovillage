use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LevelSelection, LevelSpawnBehavior, SetClearColor};

use crate::world::load_level;

pub struct LdtkClientPlugin;
impl Plugin for LdtkClientPlugin {

    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_startup_system(load_level);
    }
}