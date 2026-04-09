use crate::replication::events::on_destroy_entity::DestroyEntity;
use crate::replication::replication_manager::ReplicationManager;
use bevy::prelude::*;

#[derive(Message, Debug)]
pub struct ClientDisconnected {
    pub client_net_id: u32,
}

pub fn on_client_disconnected(
    mut messages: MessageReader<ClientDisconnected>,
    mut commands: Commands,
    replication_manager: ResMut<ReplicationManager>,
    mut ev_destroy_entity: MessageWriter<DestroyEntity>,
) {
    for on_disconnected in messages.read() {
        if let Some(client) = replication_manager
            .client_entities
            .get(&on_disconnected.client_net_id)
        {
            for entity in client.possessed_entity.values() {
                ev_destroy_entity.write(DestroyEntity { entity: *entity });
            }
            commands.entity(client.client).despawn();
        }
    }
}
