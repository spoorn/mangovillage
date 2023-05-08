use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ComputedColliderShape;
use bevy_rapier3d::prelude::{Collider, RapierDebugRenderPlugin};
use crate::networking::server_packets::LevelInfo;

use crate::state::server::ServerState;
use crate::world::load_level;

pub struct LevelServerPlugin;
impl Plugin for LevelServerPlugin {
    
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierDebugRenderPlugin::default().always_on_top())
            .add_state::<ServerState>()
            .add_system(spawn_scene.in_schedule(OnEnter(ServerState::LoadWorld)))
            .add_system(load_colliders.in_set(OnUpdate(ServerState::LoadWorld)));
    }
}

fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_level(&mut commands, &asset_server, &LevelInfo {
        handle_id: "models/volcano_island_lowpoly/scene.gltf#Scene0".to_string(),
        scene_transform: [0.0, 0.0, -20.0, std::f32::consts::PI / 2.0],
        scale: 0.005
    });
}

// TODO: Make sure only select meshes we want
fn load_colliders(mut commands: Commands, meshes: Res<Assets<Mesh>>, mesh_query: Query<(Entity, &Handle<Mesh>), Without<Collider>>, mut server_state: ResMut<NextState<ServerState>>) {
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
    info!("### meshes: {:?}", meshes);
}
