use bevy::prelude::{Entity, Resource};
use std::collections::HashMap;

#[derive(Resource)]
pub struct NetworkMapping {
    // Mapping NetworkID -> Entity Bevy
    pub map: HashMap<u32, Entity>,
}
