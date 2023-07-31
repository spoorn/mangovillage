fn main() {
    // // Allow spinning up as either client or server through same program.  We can split it up later.
    // let args: Vec<String> = env::args().collect();
    // let client_or_server = if args.len() >= 2 { &args[1] } else { "client" };
    // let client_addr = if args.len() >= 3 { args[2].to_owned() } else { "0.0.0.0:5001".to_string() };
    // let server_addr = if args.len() >= 4 { args[3].to_owned() } else { "192.168.1.243:28154".to_string() };
    // // cargo run --release -- client 0.0.0.0:5002
    //
    // // https://github.com/bevyengine/bevy/issues/1969 - cannot add LogPlugin more than once
    //
    // if client_or_server == "server" || client_or_server == "both" {
    //     println!("[server] Initializing server");
    //     let server_addr = server_addr.clone();
    //     let server_run = move || {
    //         println!("[server] Server thread spawned");
    //         // Keep server alive
    //         loop {
    //             App::new()
    //                 // Use MinimalPlugins when bevy_ecs_ldtk can disable rendering features
    //                 .add_plugins(DefaultPlugins.build()
    //                     // .add(WindowPlugin::default())
    //                     // .add(WinitPlugin::default())
    //                     // .add(AssetPlugin::default())
    //                     // .add(RenderPlugin::default())
    //                     // .add(ImagePlugin::default())
    //                     // .add(CorePipelinePlugin::default())
    //                     // .add(SpritePlugin::default())
    //                     // .add(TransformPlugin::default())
    //                     // .add(HierarchyPlugin::default())
    //                     // .add(DiagnosticsPlugin::default())
    //                     .set( LogPlugin {
    //                         filter: "info,mangovillage=debug,durian=info,wgpu=error".to_string(),
    //                         level: Level::INFO
    //                     })
    //                 )
    //                 // So both client and server can be ran at once without blocking
    //                 .insert_resource(WinitSettings {
    //                     return_from_run: true,
    //                     focused_mode: UpdateMode::Continuous,
    //                     unfocused_mode: UpdateMode::Continuous
    //                 })
    //                 .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    //                 .insert_resource(RapierConfiguration {
    //                     gravity: Vec3::new(0.0, 0.0, -0.1),
    //                     ..default()
    //                 })
    //                 .add_plugin(server::server::ServerPlugin { server_addr: server_addr.clone() })
    //                 .add_plugin(player::server::PlayerServerPlugin)
    //                 .add_plugin(world::server::LevelServerPlugin)
    //                 .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
    //                 .add_plugin(debug::server::DebugServerPlugin)
    //                 .run();
    //         }
    //     };
    //     if client_or_server == "server" {
    //         server_run();
    //     } else {
    //         thread::spawn(server_run);
    //     }
    // }
    //
    // // "both" to spin up the server on a separate background thread, and client on main thread
    // if client_or_server == "client" || client_or_server == "both" {
    //     println!("[client] Initializing client");
    //     hide_console_window();
    //     let default_plugins = if client_or_server == "both" {
    //         DefaultPlugins.build().disable::<LogPlugin>()
    //     } else {
    //         DefaultPlugins.build().set(LogPlugin {
    //             filter: "info,mangovillage=debug,durian=info,wgpu=error".to_string(),
    //             level: Level::INFO
    //         })
    //     };
    //     App::new()
    //         //.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    //         .add_plugins(default_plugins.set(ImagePlugin::default_nearest())
    //             .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
    //             .set({
    //                 WindowPlugin {
    //                     primary_window: Some(Window {
    //                         title: "Mango Village".to_string(),
    //                         resolution: WindowResolution::new(1280.0, 720.0),
    //                         position: WindowPosition::Centered(MonitorSelection::Primary),
    //                         ..default()
    //                     }),
    //                     ..default()
    //                 }
    //         }))
    //         .add_plugin(camera::CameraPlugin)
    //         .add_plugin(client::client::ClientPlugin { client_addr: client_addr.clone(), server_addr: server_addr.clone() })
    //         .add_plugin(player::client::PlayerClientPlugin)
    //         .add_plugin(world::client::WorldClientPlugin)
    //         .add_plugin(debug::client::DebugClientPlugin)
    //         .run();
    // }
}
