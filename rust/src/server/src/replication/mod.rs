use crate::SERVER_FREQUENCY;
use crate::replication::events::on_client_connected::{ClientConnected, on_client_connected};
use crate::replication::events::on_client_disconnected::{
    ClientDisconnected, on_client_disconnected,
};
use crate::replication::replication_manager::{ReplicationManager, handle_snapshots};
use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::prelude::{Fixed, Time};
use std::collections::HashMap;

pub mod events;
pub mod replicated_nodes;
pub mod replication_manager;

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplicationManager {
            client_entities: HashMap::new(),
        })
        .add_message::<ClientConnected>()
        .add_message::<ClientDisconnected>()
        .add_systems(Update, (on_client_connected, on_client_disconnected))
        .insert_resource(Time::<Fixed>::from_hz(SERVER_FREQUENCY))
        .add_systems(FixedUpdate, handle_snapshots);
    }
}
