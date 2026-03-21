use crate::network_manager::GDNetworkManager;
use common::input_packet::{Input, InputPacket};
use common::message_header::MessageType;
use common::stream_writer::StreamWriter;
use godot::builtin::Vector2;
use godot::classes::Node;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, godot_print, GodotClass, INode};
use std::collections::VecDeque;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDInputManager {
    pub base: Base<Node>,

    network_manager: Option<Gd<GDNetworkManager>>,
    input_packets: VecDeque<InputPacket>,
}

#[godot_api]
impl INode for GDInputManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            network_manager: None,
            input_packets: VecDeque::new(),
        }
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
    }
}

#[godot_api]
impl GDInputManager {
    #[func]
    pub fn send_input(&mut self, net_id: u32, direction: Vector2) {
        let mut input_packet = InputPacket::new(net_id);

        if direction.y > 0.0 {
            input_packet.add_input(Input::Up)
        }
        if direction.y < 0.0 {
            input_packet.add_input(Input::Down)
        }
        if direction.x > 0.0 {
            input_packet.add_input(Input::Right)
        }
        if direction.x < 0.0 {
            input_packet.add_input(Input::Left)
        }

        if let Some(last_packet) = self.input_packets.iter().last() {
            input_packet.sequence = last_packet.sequence + 1;
        }

        self.input_packets.push_back(input_packet.clone());

        if self.input_packets.len() > 20 {
            self.input_packets.pop_front();
        }

        if let Some(network_manager) = &mut self.network_manager {
            let mut stream_writer = StreamWriter::new();
            let vec_inputs = Vec::from(self.input_packets.clone());
            godot_print!("Stream Length: {}", vec_inputs.len());
            godot_print!("First packet : {}", vec_inputs.first().unwrap().sequence);
            stream_writer.write_serializable_vec(vec_inputs);
            network_manager
                .bind()
                .send_message(MessageType::Data, &mut stream_writer.get_data().to_vec());
        }
    }
}
