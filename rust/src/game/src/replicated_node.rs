use godot::classes::Node;
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass, INode};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDReplicatedNode {
    base: Base<Node>,

    #[export]
    pub net_id: u32,
}

#[godot_api]
impl INode for GDReplicatedNode {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base, net_id: 0 }
    }
}

#[godot_api]
impl GDReplicatedNode {
    #[signal]
    pub fn deserialize(snap1: Vec<u8>, snap2: Vec<u8>, alpha: f32);
}
