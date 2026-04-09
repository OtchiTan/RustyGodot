use crate::SERVER_IP;
use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::events::on_client_connected::ClientConnected;
use crate::replication::events::on_client_disconnected::ClientDisconnected;
use crate::replication::replicated_nodes::player::Player;
use crate::rpc::input_manager::InputManager;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::ping_request::{PingRequest, PingResponse};
use common::stream_writer::StreamWriter;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod connected_client;
pub mod network_manager;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new(SERVER_IP))
            .add_message::<PingReceived>()
            .add_systems(Update, on_ping_received)
            .insert_resource(Time::<Fixed>::from_hz(30.0))
            .add_systems(FixedUpdate, poll)
            .insert_resource(Time::<Fixed>::from_hz(1.0))
            .add_systems(FixedUpdate, handle_timeout);
    }
}

fn handle_timeout(connected_clients: Query<&ConnectedClient>, mut commands: Commands) {
    let server_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    for client in connected_clients.iter() {
        let rtt = Duration::from_millis(server_time - client.latest_ping).as_millis();
        if rtt > 300 {
            //println!("Timed out client {}", client.net_id);
            //commands.trigger(ClientDisconnected {
            //    client_net_id: client.net_id,
            //})
        }
    }
}

fn poll(
    commands: Commands,
    network_manager: ResMut<NetworkManager>,
    players: Query<&mut Player>,
    mut input_manager: ResMut<InputManager>,
    ev_ping_received: MessageWriter<PingReceived>,
    ev_client_connected: MessageWriter<ClientConnected>,
    ev_client_disconnected: MessageWriter<ClientDisconnected>,
) {
    let poll_events = network_manager.poll(
        commands,
        ev_ping_received,
        ev_client_connected,
        ev_client_disconnected,
    );
    input_manager.handle_input(poll_events, players);
}

#[derive(Message, Debug)]
pub struct PingReceived {
    ping_request: PingRequest,
    address: String,
}

fn on_ping_received(
    mut messages: MessageReader<PingReceived>,
    network_manager: Res<NetworkManager>,
    mut connected_clients: Query<&mut ConnectedClient>,
    input_manager: Res<InputManager>,
) {
    for ping_received in messages.read() {
        let server_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if let Some(mut connected_client) = connected_clients
            .iter_mut()
            .find(|client| client.address == ping_received.address)
        {
            connected_client.latest_ping = server_time;
        }

        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Ping, DataType::None));
        let ping_response = PingResponse {
            time_client_request: ping_received.ping_request.time_client_request,
            time_server_response: server_time,
            server_frame: input_manager.server_frame,
        };
        stream_writer.write_serializable(ping_response);

        network_manager.send_data(&ping_received.address, stream_writer.get_data())
    }
}
