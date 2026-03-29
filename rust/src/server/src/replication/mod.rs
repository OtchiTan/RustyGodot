use crate::replication::events::on_client_connected::on_client_connected;
use crate::replication::events::on_client_disconnected::on_client_disconnected;
use crate::replication::events::on_destroy_entity::on_destroy_entity;
use crate::replication::replication_manager::{update_replication, ReplicationManager};
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{Fixed, Time};
use std::collections::HashMap;

pub mod events;
pub mod replication_manager;
pub mod replicated_nodes;

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplicationManager {
            client_entities: HashMap::new(),
        })
        .add_observer(on_destroy_entity)
        .add_observer(on_client_connected)
        .add_observer(on_client_disconnected)
        .insert_resource(Time::<Fixed>::from_hz(30.0))
        .add_systems(FixedUpdate, update_replication);
    }
}
