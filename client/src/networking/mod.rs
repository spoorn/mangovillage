mod resource;

use std::time::Duration;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowCloseRequested;
use durian::{ClientConfig, PacketManager, register_receive, register_send};
use mangovillage_common::networking::client_packets::{Connect, Disconnect};
use mangovillage_common::networking::server_packets::{ConnectAck, ConnectAckPacketBuilder};
use mangovillage_common::util;
use crate::networking::resource::{ClientInfo, ClientPacketManager};

pub struct ClientPlugin {
    pub client_addr: String,
    pub server_addr: String
}

impl Plugin for ClientPlugin {

    fn build(&self, app: &mut App) {
        app.insert_resource(ClientInfo {
            client_addr: self.client_addr.clone(),
            server_addr: self.server_addr.clone()
        })
            .add_systems(Startup, init_client)
            .add_systems(Update, on_app_exit);
    }
}

fn init_client(mut commands: Commands, client_info: Res<ClientInfo>) {
    let mut manager = PacketManager::new();
    // register packets client-side
    let receives = util::validate_register_results(true, register_receive!(manager, (ConnectAck, ConnectAckPacketBuilder)));
    let sends = util::validate_register_results(true, register_send!(manager, Connect, Disconnect));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    let mut client_config = ClientConfig::new(client_info.client_addr.clone(), client_info.server_addr.clone(), 1, 2);
    // Server sends keep alive packets
    client_config.with_keep_alive_interval(Duration::from_secs(30));
    manager.init_client(client_config).unwrap();

    info!("[client] Initialized client");
    manager.send(Connect).unwrap();
    commands.insert_resource(ClientPacketManager { manager });
}

// Send disconnect packet to server to disconnect gracefully rather than wait for timeout.
fn on_app_exit(mut manager: ResMut<ClientPacketManager>, exit: EventReader<AppExit>, close_window: EventReader<WindowCloseRequested>) {
    if !exit.is_empty() || !close_window.is_empty() {
        info!("[client] Exiting game");
        manager.send(Disconnect).unwrap();
    }
}