use bevy::app::App;
use bevy::prelude::{Assets, AssetServer, Commands, default, Handle, info, Plugin, Query, Res, ResMut, State, SystemSet, Transform};
use bevy_ecs_ldtk::{LdtkAsset, LdtkLevel, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSet, LevelSpawnBehavior};
use bevy_ecs_ldtk::ldtk::NeighbourLevel;

use crate::state::server::ServerState;
use crate::world::LEVEL_IIDS;
use crate::world::resources::{Map, World};

pub struct LdtkServerPlugin;
impl Plugin for LdtkServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_state(ServerState::Loading)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_startup_system(load_level)
            .add_system_set(SystemSet::on_update(ServerState::Loading)
                .with_system(cache_world)
            );
    }
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("[server] Loading LDTK level");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/test.ldtk"),
        level_set: LevelSet { iids: LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect() },
        ..default()
    });
}

fn cache_world(mut commands: Commands, level_query: Query<(&Transform, &Handle<LdtkAsset>)>, ldtk_assets: Res<Assets<LdtkAsset>>, ldtk_levels: Res<Assets<LdtkLevel>>, mut server_state: ResMut<State<ServerState>>) {
    let mut world = World::default();
    let (level_transform, ldtk_asset_handle) = level_query.single();
    let ldtk_asset = ldtk_assets.get(ldtk_asset_handle).unwrap();
    for (iid, level_handle) in &ldtk_asset.level_map {
        let level = &ldtk_levels.get(&level_handle).unwrap().level;
        let base_x = level_transform.translation.x;
        let base_y = level_transform.translation.y;
        assert!(level.neighbours.len() < 5, "Level {} has more than 4 neighbors", level.iid);
        world.maps.insert(iid.clone(), Map {
            bounds: [base_x, base_x + level.px_wid as f32, base_y, base_y + level.px_hei as f32],
            neighbors: level.neighbours.iter().map(|neighbor| (neighbor.dir.clone(), neighbor.level_iid.clone())).collect()
        });
    }
    info!("[server] Loaded world: {:#?}", world);
    commands.insert_resource(world);
    server_state.set(ServerState::Running).unwrap();
}

