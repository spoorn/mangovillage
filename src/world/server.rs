use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LevelSelection, LevelSpawnBehavior};

use crate::world::load_level;

pub struct LdtkServerPlugin;
impl Plugin for LdtkServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_startup_system(load_level);
    }
}

