use bevy::prelude::Component;

#[derive(Component)]
pub struct ConnectedClient {
    pub net_id: u32,
    pub address: String,
    pub latest_ping: u64,
}
