use bevy::asset::LoadState;
use bevy::ecs::schedule::Condition;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::state::ClientState;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RapierPhysicsPlugin::<NoUserData>::default(), RapierDebugRenderPlugin::default()))
            .add_systems(Update, load_colliders.run_if(in_state(ClientState::LoadingLevel)));
    }
}

// TODO: optimize
fn load_colliders(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<(Entity, &Handle<Mesh>)>,
    mut client_state: ResMut<NextState<ClientState>>,
    mut counter: Local<u32>,
) {
    // let mesh = meshes.get(&asset_server.load("models/volcano_island_lowpoly/scene.gltf#Mesh0/Primitive0"));
    let mut done = false;
    for (entity, mesh) in &mesh_query {
        let collider = Collider::from_bevy_mesh(meshes.get(mesh).unwrap(), &ComputedColliderShape::TriMesh);
        if let Some(collider) = collider {
            commands.entity(entity).insert(collider);
            done = true;
        }
    }
    println!("done={}, counter={}", done, *counter);
    if done {
        *counter += 1;
    }
    if *counter >= 2 {
        println!("set to running");
        client_state.set(ClientState::Running);
    }
}
