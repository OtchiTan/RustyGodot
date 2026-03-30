use crate::SERVER_IP;
use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::events::on_client_connected::ClientConnected;
use crate::replication::events::on_client_disconnected::ClientDisconnected;
use crate::rpc::rpc_manager::InputReceived;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_writer::StreamWriter;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod connected_client;
pub mod network_manager;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new(SERVER_IP))
            .add_observer(on_ping_received)
            .add_systems(Update, poll)
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
            println!("Timed out client {}", client.net_id);
            commands.trigger(ClientDisconnected {
                client_net_id: client.net_id,
            })
        }
    }
}

pub enum PollEvent {
    None,
    Ping(PingReceived),
    Connected(ClientConnected),
    Disconnected(ClientDisconnected),
    Input(InputReceived),
}

fn poll(world: &mut World) {
    let mut poll_event = PollEvent::None;
    world.resource_scope(|world, network_manager: Mut<NetworkManager>| {
        poll_event = network_manager.poll(world);
    });

    match poll_event {
        PollEvent::None => {}
        PollEvent::Ping(ping_received) => world.trigger(ping_received),
        PollEvent::Connected(connected_client) => world.trigger(connected_client),
        PollEvent::Disconnected(disconnected_client) => world.trigger(disconnected_client),
        PollEvent::Input(input_received) => world.trigger(input_received),
    }
}

#[derive(Event)]
pub struct PingReceived {
    client_time: u64,
    address: String,
}

fn on_ping_received(
    on_ping_received: On<PingReceived>,
    network_manager: Res<NetworkManager>,
    mut connected_clients: Query<&mut ConnectedClient>,
) {
    let server_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    if let Some(mut connected_client) = connected_clients
        .iter_mut()
        .find(|client| client.address == on_ping_received.address)
    {
        connected_client.latest_ping = server_time;
    }

    let mut stream_writer = StreamWriter::new();
    stream_writer.write_serializable(MessageHeader::init(MessageType::Ping, DataType::None));
    stream_writer.write_u64(on_ping_received.client_time);
    stream_writer.write_u64(server_time);

    network_manager.send_data(&on_ping_received.address, stream_writer.get_data())
}
