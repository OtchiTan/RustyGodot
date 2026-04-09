use crate::network_manager::GDNetworkManager;
use crate::replicated_node::GDReplicatedNode;
use common::input_packet::{Input, InputBuffer, InputPacket};
use common::message_header::MessageType;
use common::stream_writer::StreamWriter;
use godot::builtin::Vector2;
use godot::classes::Node;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass, INode};
use std::collections::VecDeque;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDInputManager {
    pub base: Base<Node>,

    network_manager: Option<Gd<GDNetworkManager>>,
    input_packets: VecDeque<InputPacket>,
    current_input: InputPacket,
    net_id: u32,
}

#[godot_api]
impl INode for GDInputManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            network_manager: None,
            input_packets: VecDeque::new(),
            current_input: InputPacket::new(),
            net_id: 0,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if let Some(network_manager) = &mut self.network_manager {
            self.current_input.sequence = network_manager.bind().get_server_frame();

            self.input_packets.push_back(self.current_input.clone());

            if self.input_packets.len() > 20 {
                self.input_packets.pop_front();
            }

            let mut stream_writer = StreamWriter::new();
            let input_buffer = InputBuffer {
                node_id: self.net_id,
                packets: Vec::from(self.input_packets.clone()),
            };
            stream_writer.write_serializable(input_buffer);
            network_manager
                .bind()
                .send_message(MessageType::Data, &mut stream_writer.get_data().to_vec());
        }

        self.current_input.reset();
    }

    fn ready(&mut self) {
        self.network_manager = Some(
            self.base()
                .get_tree()
                .get_nodes_in_group("Network")
                .get(0)
                .unwrap()
                .cast(),
        );

        if let Some(replicated_node) = self.base().get_parent() {
            self.net_id = replicated_node.cast::<GDReplicatedNode>().bind().net_id;
        }
    }
}

#[godot_api]
impl GDInputManager {
    #[func]
    pub fn add_direction_input(&mut self, direction: Vector2) {
        if direction.y > 0.0 {
            self.current_input.add_input(Input::Up)
        }
        if direction.y < 0.0 {
            self.current_input.add_input(Input::Down)
        }
        if direction.x > 0.0 {
            self.current_input.add_input(Input::Right)
        }
        if direction.x < 0.0 {
            self.current_input.add_input(Input::Left)
        }
    }
}
