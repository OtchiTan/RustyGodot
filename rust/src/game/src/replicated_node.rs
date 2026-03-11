use godot::classes::Node;
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass, INode};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDReplicatedNode {
    base: Base<Node>,

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
    pub fn deserialize(bytes: Vec<u8>);
}