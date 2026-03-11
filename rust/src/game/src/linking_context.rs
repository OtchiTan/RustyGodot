use crate::replicated_node::GDReplicatedNode;
use godot::classes::{INode, Node, PackedScene};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, Array, GodotClass};
use std::collections::HashMap;
use common::stream_reader::StreamReader;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDLinkingContext {
    #[export]
    pub scenes_links: Array<Gd<PackedScene>>,

    replicated_nodes: HashMap<u32, Gd<GDReplicatedNode>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for GDLinkingContext {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            scenes_links: Array::new(),
            replicated_nodes: HashMap::new(),
        }
    }
}

#[godot_api]
impl GDLinkingContext {
    pub fn handle_snapshot(&mut self, buffer: Vec<u8>) {
        let mut stream_reader = StreamReader::new(buffer.to_vec());
        let net_id = stream_reader.read_u32();

        let replicated_node = self.get_replicated_node(net_id);
        if let Some(replicated_node) = replicated_node {
            replicated_node
                .signals()
                .deserialize()
                .emit(buffer[8..].to_vec());
        } else {
            self.spawn(buffer);
        }
    }

    pub fn spawn(&mut self, buffer: Vec<u8>) {
        let mut stream_reader = StreamReader::new(buffer.to_vec());
        let net_id = stream_reader.read_u32();
        let type_id = stream_reader.read_u32();

        if let Some(scene) = &self.scenes_links.get(type_id as usize) {
            let mut replicated_node = scene.instantiate_as::<GDReplicatedNode>();

            replicated_node.bind_mut().net_id = net_id;

            self.replicated_nodes
                .insert(net_id, replicated_node.clone());

            self.base_mut().add_child(&replicated_node);

            replicated_node
                .signals()
                .deserialize()
                .emit(buffer[8..].to_vec());
        }
    }

    pub fn despawn(&mut self, net_id: u32) {
        if let Some(replicated_node) = self.replicated_nodes.get_mut(&net_id) {
            replicated_node.queue_free();
        }
        self.replicated_nodes.remove(&net_id);
    }

    pub fn get_replicated_node(&self, net_id: u32) -> Option<Gd<GDReplicatedNode>> {
        if let Some(replicated_node) = self.replicated_nodes.get(&net_id) {
            return Some(replicated_node.clone());
        }
        None
    }
}
