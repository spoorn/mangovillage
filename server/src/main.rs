use std::env;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::window::ExitCondition;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use crate::state::ServerState;

mod networking;
mod physics;
mod player;
mod state;
mod world;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_server_addr = "127.0.0.1:28154".to_string();
    let server_addr = args.get(1).unwrap_or(&default_server_addr);
    println!("[server] Initializing server");

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(LogPlugin { filter: "info,mangovillage_server=debug,durian=info,wgpu=error".to_string(), level: Level::INFO })
                // If we disable WinitPlugin, we'll end up returning from App::run() immediately.  Disable it for true headless
                //.disable::<WinitPlugin>()
                // headless mode, with no rendering backends and no window
                .set(RenderPlugin { wgpu_settings: WgpuSettings { backends: None, ..default() } })
                .set(WindowPlugin { primary_window: None, exit_condition: ExitCondition::DontExit, close_when_requested: false, ..default() })
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        // So both client and server can be ran at once without blocking
        // .insert_resource(WinitSettings {
        //     return_from_run: true,
        //     focused_mode: UpdateMode::Continuous,
        //     unfocused_mode: UpdateMode::Continuous
        // })
        .add_state::<ServerState>()
        .add_plugins((networking::ServerPlugin { server_addr: server_addr.clone() }, world::WorldPlugin, physics::PhysicsPlugin))
        .run();
}
