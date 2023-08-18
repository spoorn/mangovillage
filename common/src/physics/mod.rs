use bevy::prelude::{Assets, Commands, Entity, Handle, Mesh, Res};
use bevy_xpbd_3d::prelude::{Collider, RigidBody};

pub mod component;

/// Spawn colliders for meshes
///
/// See https://stackoverflow.com/questions/35592750/how-does-for-syntax-differ-from-a-regular-lifetime-bound,
/// https://stackoverflow.com/questions/76151501/storing-an-iterator-over-borrowed-refs-inside-a-struct, and
/// https://github.com/rust-lang/rust/issues/49601 for explanation on higher ranked trait bounds/lifetimes and
/// why it couldn't be used here
pub fn spawn_colliders<'a, I>(commands: &mut Commands, meshes: &Res<Assets<Mesh>>, mesh_query: I) -> bool
where
    I: Iterator<Item = (Entity, &'a Handle<Mesh>)>,
{
    let mut done = false;
    for (entity, mesh) in mesh_query {
        let collider = Collider::trimesh_from_bevy_mesh(meshes.get(&mesh).unwrap());
        if let Some(collider) = collider {
            commands.entity(entity).insert(RigidBody::Static).insert(collider);
            done = true;
        }
    }
    done
}
