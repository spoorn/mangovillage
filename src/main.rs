use std::{env, thread};
use bevy::app::App;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowDescriptor;
use bevy::winit::{UpdateMode, WinitSettings};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod networking;
mod client;
mod server;
mod player;
mod camera;
mod map;

// For hiding the console window on client mode
fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

fn main() {
    // Allow spinning up as either client or server through same program.  We can split it up later.
    let args: Vec<String> = env::args().collect();
    let client_or_server = if args.len() >= 2 { &args[1] } else { "client" };
    let server_addr = if args.len() >= 3 { args[2].to_owned() } else { "192.168.1.243:28154".to_string() };
    let client_addr = if args.len() >= 4 { args[3].to_owned() } else { "0.0.0.0:5001".to_string() };
    
    // https://github.com/bevyengine/bevy/issues/1969 - cannot add LogPlugin more than once

    if client_or_server == "server" || client_or_server == "both" {
        println!("[server] Initializing server");
        thread::spawn(move || {
            println!("[server] Server thread spawned");
            // Keep server alive
            loop {
                App::new()
                    .add_plugins(MinimalPlugins.build()
                        .add(InputPlugin::default())
                        .add(TransformPlugin::default())
                        .add(HierarchyPlugin::default())
                        .add(DiagnosticsPlugin::default())
                        .add(LogPlugin::default())
                    )
                    // So both client and server can be ran at once without blocking
                    .insert_resource(WinitSettings {
                        return_from_run: true,
                        focused_mode: UpdateMode::Continuous,
                        unfocused_mode: UpdateMode::Continuous
                    })
                    .run();
            }
        });
    }
    
    // "both" to spin up the server on a separate background thread, and client on main thread
    if client_or_server == "client" || client_or_server == "both" {
        println!("[client] Initializing client");
        hide_console_window();
        let default_plugins = if client_or_server == "both" {
            DefaultPlugins.build().disable::<LogPlugin>()
        } else {
            DefaultPlugins.build()
        };
        App::new()
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .add_plugins(default_plugins
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
                .set({
                    WindowPlugin {
                        window: WindowDescriptor {
                            title: "Mango Village".to_string(),
                            width: 1000.0,
                            height: 800.0,
                            position: WindowPosition::Centered,
                            monitor: MonitorSelection::Current,
                            ..default()
                        },
                        ..default()
                    }
            }))
            .add_plugin(camera::CameraPlugin)
            .add_plugin(player::client::PlayerClientPlugin)
            .run();
    }
}
