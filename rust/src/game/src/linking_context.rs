use crate::replicated_node::GDReplicatedNode;
use common::snapshot::Snapshot;
use godot::classes::{INode, Node, PackedScene};
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
    pub fn handle_snapshot(&mut self, snap1: Snapshot, snap2: Snapshot, alpha: f32) {
        for node in snap1.nodes {
            let next_frame_node = snap2
                .nodes
                .iter()
                .find(|next_node| next_node.net_id == node.net_id);

            if let Some(next_frame_node) = next_frame_node {
                let replicated_node = self.get_replicated_node(node.net_id);

                if let Some(replicated_node) = replicated_node {
                    replicated_node.signals().deserialize().emit(
                        node.data,
                        next_frame_node.data.clone(),
                        alpha,
                    );
                } else {
                    self.spawn(next_frame_node.net_id, next_frame_node.type_id);
                }
            }
        }
    }

    pub fn spawn(&mut self, net_id: u32, type_id: u32) {
        if let Some(scene) = &self.scenes_links.get(type_id as usize) {
            let mut replicated_node = scene.instantiate_as::<GDReplicatedNode>();

            replicated_node.bind_mut().net_id = net_id;

            self.replicated_nodes
                .insert(net_id, replicated_node.clone());

            self.base_mut().add_child(&replicated_node);
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
