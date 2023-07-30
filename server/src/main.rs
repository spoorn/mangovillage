mod networking;

use std::env;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_server_addr = "127.0.0.1:28154".to_string();
    let server_addr = args.get(1).unwrap_or(&default_server_addr);
    println!("[server] Initializing server");

    loop {
        App::new()
            .add_plugins(MinimalPlugins.build()
                .add( LogPlugin {
                    filter: "info,mangovillage_server=debug,durian=info,wgpu=error".to_string(),
                    level: Level::INFO
                })
            )
            // So both client and server can be ran at once without blocking
            // .insert_resource(WinitSettings {
            //     return_from_run: true,
            //     focused_mode: UpdateMode::Continuous,
            //     unfocused_mode: UpdateMode::Continuous
            // })
            .add_plugins(networking::ServerPlugin { server_addr: server_addr.clone() })
            .run();
    }
}
