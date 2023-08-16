use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        // This sets image filtering to nearest
        // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
        // by linear filtering.
        .add_plugins(DefaultPlugins.build().add_before::<AssetPlugin, _>(EmbeddedAssetPlugin).set({
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Mango Village".to_string(),
                    resolution: WindowResolution::new(1280.0, 720.0),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        }))
        .add_state::<ClientState>()
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, (load_world, setup_camera))
        .add_systems(Update, load_colliders.run_if(in_state(ClientState::LoadingLevel)))
        .run();
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    LoadingLevel,
    Running,
}

fn setup_camera(mut commands: Commands) {
    let camera_translation = Vec3::new(0.0, 50000.0, 0.0);
    let focus = Vec3::new(0.0, 0.0, 0.0);
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(camera_translation).looking_at(focus, Vec3::Y),
        ..default()
    },));
}

/// Loads a scene
fn load_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client_state: ResMut<NextState<ClientState>>,
) {
    let scene_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    commands.spawn(SceneBundle {
        scene: asset_server.load(&"models/volcano_island_lowpoly/scene.gltf#Scene0".to_string()),
        transform: scene_transform,
        ..default()
    });
    client_state.set(ClientState::LoadingLevel);
}

/// Loads colliders, but the colliders don't match the mesh scale
fn load_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<(Entity, &Handle<Mesh>), Without<Collider>>,
    mut client_state: ResMut<NextState<ClientState>>,
) {
    let mut done = false;
    for (entity, mesh) in &mesh_query {
        let collider = Collider::trimesh_from_bevy_mesh(meshes.get(mesh).unwrap());
        if let Some(collider) = collider {
            commands.entity(entity).insert(collider);
            done = true;
        }
    }
    if done {
        client_state.set(ClientState::Running);
    }
}
