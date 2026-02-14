use bevy::prelude::Component;

#[derive(Component)]
pub struct ReplicatedNode {
    pub net_id: u32,
    pub x: f32,
    pub y: f32,
}

impl Clone for ReplicatedNode {
    fn clone(&self) -> Self {
        ReplicatedNode {
            net_id: self.net_id,
            x: self.x,
            y: self.y,
        }
    }
}
