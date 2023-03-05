use bevy::prelude::{AssetServer, Commands, default, Res};
use bevy_ecs_ldtk::LdtkWorldBundle;

pub mod client;
pub mod server;

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/test.ldtk"),
        ..default()
    });
}