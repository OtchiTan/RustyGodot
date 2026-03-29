
use crate::replication::replication_manager::ReplicationManager;
use bevy::prelude::*;
use crate::replication::events::on_destroy_entity::DestroyEntity;

#[derive(Event)]
pub struct ClientDisconnected {
    pub client_net_id: u32,
}

pub fn on_client_disconnected(
    on_disconnected: On<ClientDisconnected>,
    mut commands: Commands,
    replication_manager: ResMut<ReplicationManager>,
) {
    if let Some(client) = replication_manager
        .client_entities
        .get(&on_disconnected.client_net_id)
    {
        for entity in client.possessed_entity.values() {
            commands.trigger(DestroyEntity { entity: *entity });
        }
        commands.entity(client.client).despawn();
    }
}
