use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LayerMetadata, LdtkAsset, LdtkLevel, LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSet, LevelSpawnBehavior};
use bevy_ecs_ldtk::prelude::{LdtkEntityAppExt, LdtkIntCellAppExt};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::ComputedColliderShape;
use bevy_rapier3d::prelude::{Collider, Friction, LockedAxes, RapierDebugRenderPlugin};

use crate::common::components::{ColliderBundle, Position};
use crate::state::server::ServerState;
use crate::world::{GRID_SIZE, LEVEL_IIDS, util};
use crate::world::components::{PlayerSpawn, PlayerSpawnBundle, Portal, PortalBundle, Wall, WallBundle};
use crate::world::resources::{Map, PortalInfo, World};

pub struct LdtkServerPlugin;
impl Plugin for LdtkServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_plugin(RapierDebugRenderPlugin::default().always_on_top())
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
            .register_ldtk_entity::<PortalBundle>("Portal")
            .add_state::<ServerState>()
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_startup_system(load_level)
            .add_system(cache_world.in_set(OnUpdate(ServerState::LoadWorld)))
            .add_system(load_entities.in_set(OnUpdate(ServerState::LoadEntities)))
            //.add_system(spawn_wall_colliders.in_set(OnUpdate(ServerState::LoadWalls)))
            .add_system(spawn_scene.in_schedule(OnEnter(ServerState::LoadedWorld)))
            .add_system(tst.in_set(OnUpdate(ServerState::LoadedWorld)));
            //.add_system(loaded_world.in_set(OnUpdate(ServerState::LoadedWorld)));
    }
}

fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut scene_transform = Transform::from_xyz(0.0, 0.0, 10.0).with_scale(Vec3::splat(0.001));
    scene_transform.rotate_x(std::f32::consts::PI / 2.0);
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/volcano_island_lowpoly/scene.gltf#Scene0"),
        transform: scene_transform,
        visibility: Visibility::Hidden,
        ..default()
    });
}

fn tst(mut commands: Commands, meshes: Res<Assets<Mesh>>, mesh_query: Query<(Entity, &Handle<Mesh>), Without<Collider>>, asset_server: Res<AssetServer>, mut server_state: ResMut<NextState<ServerState>>) {
    let mut done = false;
   // let mesh = meshes.get(&asset_server.load("models/volcano_island_lowpoly/scene.gltf#Mesh0/Primitive0"));
    for (entity, mesh) in &mesh_query {
        let collider = Collider::from_bevy_mesh(meshes.get(mesh).unwrap(), &ComputedColliderShape::TriMesh);
        if let Some(collider) = collider {
            commands.entity(entity).insert(collider);
            done = true;
        }
    }
    if done {
        server_state.set(ServerState::Running);
    }
    // if let Some(mesh) = mesh {
    //     let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh);
    //     // let mut scene_transform = Transform::from_xyz(0.0, 0.0, 10.0).with_scale(Vec3::splat(0.001));
    //     // scene_transform.rotate_x(std::f32::consts::PI / 2.0);
    //     // commands.spawn(SceneBundle {
    //     //     scene: asset_server.load("models/volcano_island_lowpoly/scene.gltf#Scene0"),
    //     //     transform: scene_transform,
    //     //     visibility: Visibility::Hidden,
    //     //     ..default()
    //     // })
    //     //     .insert(collider.unwrap());
    //     server_state.set(ServerState::Running);
    // }
    // TODO: Make sure only select meshes we want
    info!("### meshes: {:?}", meshes);
    // Mesh::search_in_children
    // let mesh: Handle<Mesh> = asset_server.load("models/volcano_island_lowpoly/scene.gltf#Mesh0/Primitive0");
    // Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh);
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("[server] Loading LDTK level");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/test.ldtk"),
        level_set: LevelSet { iids: LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect() },
        ..default()
    });
}

fn cache_world(mut commands: Commands, level_query: Query<(&Transform, &Handle<LdtkAsset>)>, ldtk_assets: Res<Assets<LdtkAsset>>, ldtk_levels: Res<Assets<LdtkLevel>>, mut server_state: ResMut<NextState<ServerState>>) {
    let mut world = World::default();
    let (level_transform, ldtk_asset_handle) = level_query.single();
    if let Some(ldtk_asset) = ldtk_assets.get(ldtk_asset_handle) {
        for (iid, level_handle) in &ldtk_asset.level_map {
            let level = &ldtk_levels.get(&level_handle).unwrap().level;
            let base_x = level_transform.translation.x + level.world_x as f32;
            // Note: flip the Y value as bevy increases in Y as we go up while LDTK decreases
            let base_y = level_transform.translation.y - level.world_y as f32;
            assert!(level.neighbours.len() < 5, "Level {} has more than 4 neighbors", level.iid);
            world.maps.insert(iid.clone(), Map {
                bounds: [base_x, base_x + level.px_wid as f32, base_y, base_y + level.px_hei as f32],
                neighbors: level.neighbours.iter().map(|neighbor| (neighbor.dir.clone(), neighbor.level_iid.clone())).collect(),
                world_coords: (level.world_x as f32, -level.world_y as f32),
                ..default()
            });
        }
        
        assert_eq!(LEVEL_IIDS.len(), world.maps.keys().len(), "LEVEL_IIDS is missing some levels from LDTK world!");
        info!("[server] Cached world");
        commands.insert_resource(world);
        server_state.set(ServerState::LoadEntities);
    }
}

// TODO: LdtkEntities are not available to query in the first iteration for some reason.  So we need to keep checking
// until it's available.
fn load_entities(player_spawns_query: Query<(&Transform, &Parent), With<PlayerSpawn>>, 
                 portal_query: Query<(&Transform, &Parent, &Portal)>,
                 level_query: Query<&Handle<LdtkLevel>>, 
                 ldtk_levels: Res<Assets<LdtkLevel>>, 
                 mut world: ResMut<World>, 
                 mut server_state: ResMut<NextState<ServerState>>) {
    info!("[server] Loading Entities...");
    let mut done = false;

    // TODO: Does this need to be transformed into our coordinates like for portals?
    for (transform, parent) in &player_spawns_query {
        let level_handle = level_query.get(parent.get()).unwrap();
        let level = ldtk_levels.get(level_handle).unwrap();
        // Note: flip the Y value as bevy increases in Y as we go up while LDTK decreases
        world.maps.get_mut(&level.level.iid).unwrap().player_spawn = Position::new(transform.translation.x + level.level.world_x as f32, transform.translation.y - level.level.world_y as f32);
        done = true;  // Found player spawns
    }
    
    if done {
        info!("[server] Loaded Player Spawns");
        done = false;
    } else {
        // Try next iteration, due to the LdtkEntity Components not loading on first iteration
        return;
    }
    
    for (_transform, parent, portal) in &portal_query {
        let level_handle = level_query.get(parent.get()).unwrap();
        let level = ldtk_levels.get(level_handle).unwrap();
        let destination = portal.destination.clone();
        assert!(world.maps.contains_key(&destination), "[server] Portal destination={} in Level={} does not exist!", destination, level.level.iid);
        // Transform for entities is the center of the entity, so we need width/height to get the [x1, x2, y1, y2] bounds
        let mut map_coords = util::ldtk_to_map_coordinates(GRID_SIZE, portal.ldtk_coords, level.level.px_hei);
        // Transform to world space
        map_coords.0 += level.level.world_x as f32;
        // Note: flip the Y value as bevy increases in Y as we go up while LDTK decreases
        map_coords.1 -= level.level.world_y as f32;
        //println!("{}, {}", transform.translation.x + level.level.world_x as f32, transform.translation.y + level.level.world_y as f32);
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
        info!("[server] Finished loading Entities");
        server_state.set(ServerState::LoadedWorld);
    }
}

fn spawn_wall_colliders(mut commands: Commands, wall_query: Query<(&Transform, &Wall, &Parent, Entity), Without<Collider>>, layer_metadata_query: Query<&LayerMetadata>, mut server_state: ResMut<NextState<ServerState>>) {
    info!("[server] Loading Walls...");
    let mut done = false;
    
    for (transform, _wall, parent, entity) in &wall_query {
        // Get IntGrid Layer's metadata for grid size
        let layer_metadata = layer_metadata_query.get(parent.get()).unwrap();
        // Insert colliders for wall
        commands.entity(entity)
            .insert(ColliderBundle {
                collider: Collider::cuboid(layer_metadata.grid_size as f32 / 2.0, layer_metadata.grid_size as f32 / 2.0, layer_metadata.grid_size as f32 / 2.0),
                rigid_body: RigidBody::Fixed,
                friction: Friction::new(1.0),
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..default()
            });
        done = true;
    }

    if done {
        info!("[server] Finished loading walls");
        server_state.set(ServerState::LoadedWorld);
    }
}

fn loaded_world(world: Res<World>, mut server_state: ResMut<NextState<ServerState>>) {
    info!("[server] Finished loading world: {:#?}", world);
    server_state.set(ServerState::Running);
}
