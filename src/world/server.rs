use bevy::app::App;
use bevy::prelude::{Assets, AssetServer, Commands, default, Handle, info, Parent, Plugin, Query, Res, ResMut, State, SystemSet, Transform, With};
use bevy_ecs_ldtk::{LdtkAsset, LdtkLevel, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSet, LevelSpawnBehavior};
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
use crate::common::components::Position;

use crate::state::server::ServerState;
use crate::world::components::{PlayerSpawn, PlayerSpawnBundle, Portal, PortalBundle};
use crate::world::{GRID_SIZE, LEVEL_IIDS, util};
use crate::world::resources::{Map, PortalInfo, World};

pub struct LdtkServerPlugin;
impl Plugin for LdtkServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
            .register_ldtk_entity::<PortalBundle>("Portal")
            .add_state(ServerState::LoadWorld)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_startup_system(load_level)
            .add_system_set(SystemSet::on_update(ServerState::LoadWorld)
                .with_system(cache_world)
            )
            .add_system_set(SystemSet::on_update(ServerState::LoadEntities)
                .with_system(load_entities)
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
    if let Some(ldtk_asset) = ldtk_assets.get(ldtk_asset_handle) {
        for (iid, level_handle) in &ldtk_asset.level_map {
            let level = &ldtk_levels.get(&level_handle).unwrap().level;
            let base_x = level_transform.translation.x;
            let base_y = level_transform.translation.y;
            assert!(level.neighbours.len() < 5, "Level {} has more than 4 neighbors", level.iid);
            world.maps.insert(iid.clone(), Map {
                bounds: [base_x, base_x + level.px_wid as f32, base_y, base_y + level.px_hei as f32],
                neighbors: level.neighbours.iter().map(|neighbor| (neighbor.dir.clone(), neighbor.level_iid.clone())).collect(),
                ..default()
            });
        }
        
        assert_eq!(LEVEL_IIDS.len(), world.maps.keys().len(), "LEVEL_IIDS is missing some levels from LDTK world!");
        info!("[server] Cached world");
        commands.insert_resource(world);
        server_state.set(ServerState::LoadEntities).unwrap();
    }
}

// TODO: LdtkEntities are not available to query in the first iteration for some reason.  So we need to keep checking
// until it's available.
fn load_entities(player_spawns_query: Query<(&Transform, &Parent), With<PlayerSpawn>>, 
                 portal_query: Query<(&Transform, &Parent, &Portal)>,
                 level_query: Query<&Handle<LdtkLevel>>, 
                 ldtk_levels: Res<Assets<LdtkLevel>>, 
                 mut world: ResMut<World>, 
                 mut server_state: ResMut<State<ServerState>>) {
    info!("[server] Loading Entities...");
    let mut done = false;

    for (transform, parent) in &player_spawns_query {
        let level_handle = level_query.get(parent.get()).unwrap();
        let level = ldtk_levels.get(level_handle).unwrap();
        world.maps.get_mut(&level.level.iid).unwrap().player_spawn = Position::new(transform.translation.x, transform.translation.y);
        done = true;  // Found player spawns
    }
    
    if done {
        info!("[server] Loaded Player Spawns");
        done = false;
    } else {
        // Try next iteration, due to the LdtkEntity Components not loading on first iteration
        return;
    }
    
    for (transform, parent, portal) in &portal_query {
        let level_handle = level_query.get(parent.get()).unwrap();
        let level = ldtk_levels.get(level_handle).unwrap();
        let destination = portal.destination.clone();
        assert!(world.maps.contains_key(&destination), "[server] Portal destination={} in Level={} does not exist!", destination, level.level.iid);
        // Transform for entities is the center of the entity, so we need width/height to get the [x1, x2, y1, y2] bounds
        let map_coords = util::ldtk_to_map_coordinates(GRID_SIZE, portal.ldtk_coords, level.level.px_hei);
        println!("{:?}", transform);
        world.maps.get_mut(&level.level.iid).unwrap().portals.push(PortalInfo([map_coords.0, map_coords.0 + portal.width, map_coords.1 - portal.height, map_coords.1], portal.destination.clone(), portal.link));
        done = true;
    }

    if done {
        info!("[server] Loaded Portals");
    } else {
        // Try next iteration, due to the LdtkEntity Components not loading on first iteration
        return;
    }
    
    if done {
        info!("[server] Finished loading world: {:#?}", world);
        server_state.set(ServerState::Running).unwrap();
    }
}

