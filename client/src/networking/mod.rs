pub mod resource;

use crate::networking::resource::{ClientInfo, ClientPacketManager};
use crate::player::resource::ClientId;
use crate::state::ClientState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowCloseRequested;
use durian::{register_receive, register_send, ClientConfig, PacketManager};
use mangovillage_common::networking::client_packets::{Connect, Disconnect, Movement};
use mangovillage_common::networking::server_packets::{
    ConnectAck, ConnectAckPacketBuilder, Players, PlayersPacketBuilder, SpawnScene, SpawnScenePacketBuilder,
};
use mangovillage_common::util;
use std::time::Duration;

pub struct ClientPlugin {
    pub client_addr: String,
    pub server_addr: String,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientInfo { client_addr: self.client_addr.clone(), server_addr: self.server_addr.clone() })
            .add_systems(Startup, init_client)
            .add_systems(Update, transition_running.run_if(in_state(ClientState::JoiningServer)))
            .add_systems(Update, on_app_exit);
    }
}

fn init_client(mut commands: Commands, client_info: Res<ClientInfo>) {
    let mut manager = PacketManager::new();
    // register packets client-side
    let receives = util::validate_register_results(
        true,
        register_receive!(manager, (ConnectAck, ConnectAckPacketBuilder), (SpawnScene, SpawnScenePacketBuilder), (Players, PlayersPacketBuilder)),
    );
    let sends = util::validate_register_results(true, register_send!(manager, Connect, Disconnect, Movement));
    // TODO: better error handling
    if !receives {
        panic!("Failed to register all receive packets");
    }
    if !sends {
        panic!("Failed to register all send packets");
    }
    let mut client_config = ClientConfig::new(client_info.client_addr.clone(), client_info.server_addr.clone(), 3, 3);
    // Server sends keep alive packets
    client_config.with_keep_alive_interval(Duration::from_secs(30));
    manager.init_client(client_config).unwrap();

    info!("[client] Initialized client");
    manager.send(Connect).unwrap();
    commands.insert_resource(ClientPacketManager { manager });
}

/// Waits for ConnectAck from server and goes to Running state initially, and switches states when we get commands from server
fn transition_running(mut manager: ResMut<ClientPacketManager>, mut client_state: ResMut<NextState<ClientState>>, mut commands: Commands) {
    let acks = manager.received::<ConnectAck, ConnectAckPacketBuilder>(false).unwrap();
    // Should only be 1 packet
    if let Some(acks) = acks {
        let connect_ack = acks.last().unwrap();
        info!("Received ConnectAck from server, client_id={}", connect_ack.id);
        commands.insert_resource(ClientId(connect_ack.id));
        info!("Transitioning state to LoadingLevel");
        client_state.set(ClientState::LoadingLevel);
    }
}

// Send disconnect packet to server to disconnect gracefully rather than wait for timeout.
fn on_app_exit(mut manager: ResMut<ClientPacketManager>, exit: EventReader<AppExit>, close_window: EventReader<WindowCloseRequested>) {
    if !exit.is_empty() || !close_window.is_empty() {
        info!("[client] Exiting game");
        manager.send(Disconnect).unwrap();
    }
}
