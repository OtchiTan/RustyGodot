use bevy::prelude::{Entity, Resource};
use std::collections::HashMap;

#[derive(Resource)]
pub struct ReplicationManager {
    pub map: HashMap<u32, Entity>,
}
