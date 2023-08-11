mod debug;
mod networking;
mod physics;
mod state;
mod world;

use crate::state::ClientState;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_client_addr = "0.0.0.0:5001".to_string();
    let default_server_addr = "127.0.0.1:28154".to_string();
    let client_addr = args.get(1).unwrap_or(&default_client_addr);
    let server_addr = args.get(2).unwrap_or(&default_server_addr);
    println!("[client] Initializing client");

    // Set log level manually
    let default_plugins =
        DefaultPlugins.build().set(LogPlugin { filter: "info,mangovillage_client=debug,durian=info,wgpu=error".to_string(), level: Level::INFO });

    App::new()
        // This sets image filtering to nearest
        // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
        // by linear filtering.
        .add_plugins(default_plugins.set(ImagePlugin::default_nearest()).add_before::<AssetPlugin, _>(EmbeddedAssetPlugin).set({
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
        .add_plugins((
            networking::ClientPlugin { client_addr: client_addr.clone(), server_addr: server_addr.clone() },
            world::WorldPlugin,
            physics::PhysicsPlugin,
            debug::camera::DebugCameraPlugin,
        ))
        .run();
}
