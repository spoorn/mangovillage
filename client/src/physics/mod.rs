use bevy::prelude::*;

use mangovillage_common::physics;

use crate::state::ClientState;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_xpbd_3d::prelude::PhysicsPlugins::default())
            .add_systems(Update, load_colliders.run_if(in_state(ClientState::LoadingPhysics)));
    }
}

// TODO: optimize
fn load_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<(Entity, &Handle<Mesh>)>,
    mut client_state: ResMut<NextState<ClientState>>,
    mut counter: Local<u32>,
) {
    // let mesh = meshes.get(&asset_server.load("models/volcano_island_lowpoly/scene.gltf#Mesh0/Primitive0"));
    let done = physics::spawn_colliders(&mut commands, &meshes, mesh_query.iter());
    //println!("done={}, counter={}", done, *counter);
    if done {
        *counter += 1;
    }
    if *counter >= 2 {
        info!("[client] Transitioning state to Running");
        client_state.set(ClientState::Running);
    }
}
