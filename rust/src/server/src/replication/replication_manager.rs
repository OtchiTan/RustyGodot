use bevy::prelude::{Entity, Resource};
use std::collections::HashMap;

#[derive(Resource)]
pub struct ReplicationManager {
    pub client_entities: HashMap<u32, ClientEntityLink>,
}

pub struct ClientEntityLink {
    pub client: Entity,
    pub possessed_entity: HashMap<u32, Entity>,
    pub last_sequence: u32,
}

impl ClientEntityLink {
    pub fn new(client: Entity) -> Self {
        Self {
            client,
            possessed_entity: HashMap::new(),
            last_sequence: 0,
        }
    }
}
