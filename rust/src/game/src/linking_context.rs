use crate::replicated_node::GDReplicatedNode;
use crate::stream_reader::GDStreamReader;
use common::stream_reader::StreamReader;
use godot::classes::{INode, Node, PackedScene};
use godot::obj::NewGd;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, Array, GodotClass};
use std::collections::HashMap;

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

    fn ready(&mut self) {
        self.base_mut().add_to_group("Network");
    }
}

#[godot_api]
impl GDLinkingContext {
    pub fn handle_snapshot(&mut self, buffer: Vec<u8>) {
        let mut sr = GDStreamReader::new_gd();
        sr.bind_mut().stream_reader = Some(StreamReader::new(buffer.to_vec()));
        let net_id = sr
            .bind_mut()
            .stream_reader
            .as_mut()
            .expect("Check just before")
            .read_u32();
        let type_id = sr
            .bind_mut()
            .stream_reader
            .as_mut()
            .expect("Check just before")
            .read_u32();

        let replicated_node = self.get_replicated_node(net_id);
        if let Some(replicated_node) = replicated_node {
            replicated_node.signals().deserialize().emit(&sr);
        } else {
            self.spawn(net_id, type_id, sr);
        }
    }

    pub fn spawn(&mut self, net_id: u32, type_id: u32, stream_reader: Gd<GDStreamReader>) {
        if let Some(scene) = &self.scenes_links.get(type_id as usize) {
            let mut replicated_node = scene.instantiate_as::<GDReplicatedNode>();

            replicated_node.bind_mut().net_id = net_id;

            self.replicated_nodes
                .insert(net_id, replicated_node.clone());

            self.base_mut().add_child(&replicated_node);

            replicated_node.signals().deserialize().emit(&stream_reader);
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
