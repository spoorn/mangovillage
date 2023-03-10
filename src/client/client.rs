use std::time::Duration;

use bevy::app::{App, AppExit};
use bevy::prelude::{Commands, EventReader, info, Plugin, Res, ResMut, State, SystemSet};
use bevy::window::WindowCloseRequested;
use bevy_ecs_ldtk::LevelSelection;
use durian::{ClientConfig, PacketManager, register_receive, register_send};

use crate::client::resources::{ClientId, ClientInfo, ClientPacketManager};
use crate::common::util;
use crate::networking::client_packets::{Disconnect, Move};
use crate::networking::server_packets::{ChangeLevel, ChangeLevelPacketBuilder, SpawnAck, SpawnAckPacketBuilder, UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder};
use crate::state::client::ClientState;

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
            .add_state(ClientState::JoiningServer)
            .add_startup_system(init_client)
            .add_system_set(SystemSet::on_update(ClientState::JoiningServer).with_system(get_client_id))
            .add_system(on_app_exit);
    }
}

fn init_client(mut commands: Commands, client_info: Res<ClientInfo>) {
    let mut manager = PacketManager::new();
    // register packets client-side
    let receives = util::validate_results(true, register_receive!(manager, (UpdatePlayerPositions, UpdatePlayerPositionsPacketBuilder), (SpawnAck, SpawnAckPacketBuilder), (ChangeLevel, ChangeLevelPacketBuilder)));
    let sends = util::validate_results(true, register_send!(manager, Move, Disconnect));
    // TODO: better error handling
    if !receives { panic!("Failed to register all receive packets"); }
    if !sends { panic!("Failed to register all send packets"); }
    let mut client_config = ClientConfig::new(client_info.client_addr.clone(), client_info.server_addr.clone(), 3, 2);
    // Server sends keep alive packets
    client_config.with_keep_alive_interval(Duration::from_secs(30)).with_idle_timeout(Duration::from_secs(60));
    manager.init_client(client_config).unwrap();
    
    // wait for ACK, and to get server's assigned client ID
    // TODO: There is a chance this hangs the server app so it never sends the ACK due to the sleep when running both
    // client and server at once.  Not sure why.
    commands.insert_resource(ClientId { id: 0, set: false });
    //loop {
    // if let Some(ack) = manager.received::<SpawnAck, SpawnAckPacketBuilder>(true).unwrap() {
    //     let id = ack[0].id;
    //     info!("[client] Client ID is {}", id);
    //     commands.insert_resource(ClientId { id });
    //     //break;
    // }
    //     info!("[client] Waiting for ACK from server");
    //     thread::sleep(Duration::from_secs(1));
    // }

    info!("[client] Initialized client");
    commands.insert_resource(ClientPacketManager { manager });
}

// TODO: Use states instead when bevy 0.10 with stateless RFC comes out
fn get_client_id(mut commands: Commands, mut manager: ResMut<ClientPacketManager>, mut client_id: ResMut<ClientId>, mut client_state: ResMut<State<ClientState>>) {
    if !client_id.set {
        if let Some(ack) = manager.received::<SpawnAck, SpawnAckPacketBuilder>(true).unwrap() {
            let id = ack[0].id;
            info!("[client] Client ID is {}", id);
            //commands.insert_resource(ClientId { id });
            //break;
            client_id.id = id;
            client_id.set = true;
            client_state.set(ClientState::Running).unwrap();
            
            let level_iid = ack[0].level_iid.clone();
            info!("[client] Loading level iid={}", level_iid);
            commands.insert_resource(LevelSelection::Iid(level_iid))
        }
    }
}

fn on_app_exit(mut manager: ResMut<ClientPacketManager>, exit: EventReader<AppExit>, close_window: EventReader<WindowCloseRequested>) {
    if !exit.is_empty() || !close_window.is_empty() {
        info!("[client] Exiting game");
        manager.send(Disconnect).unwrap();
    }
}