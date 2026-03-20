use crate::network_manager::GDNetworkManager;
use crate::replicated_node::GDReplicatedNode;
use common::input_packet::{Input, InputPacket};
use common::message_header::MessageType;
use common::stream_reader::{Deserializable, StreamReader};
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct GDPlayer {
    pub base: Base<CharacterBody2D>,

    owner_id: u32,
    network_manager: Option<Gd<GDNetworkManager>>,
}

#[godot_api]
impl ICharacterBody2D for GDPlayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            owner_id: 0,
            network_manager: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        if let Some(_network_manager) = &self.network_manager {}
    }

    fn ready(&mut self) {
        self.network_manager = Some(
            self.base()
                .get_parent()
                .unwrap()
                .get_parent()
                .unwrap()
                .get_node_as::<GDNetworkManager>("%GDNetworkManager"),
        );
    }
}

#[godot_api]
impl GDPlayer {
    #[func]
    pub fn is_locally_owned(&self) -> bool {
        if let Some(network_manager) = &self.network_manager {
            return self.owner_id == network_manager.bind().client_id;
        }
        false
    }

    #[func]
    pub fn send_input(&mut self, direction: Vector2) {
        if !self.is_locally_owned() {
            return;
        }

        let replicated_node = self.base().get_parent().unwrap().cast::<GDReplicatedNode>();

        let mut input_packet = InputPacket::new(replicated_node.bind().net_id);

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

        if input_packet.keys == 0 {
            return;
        }

        if let Some(network_manager) = &mut self.network_manager {
            let mut data = input_packet.serialize();
            network_manager
                .bind()
                .send_message(MessageType::Data, &mut data);
        }
    }

    #[func]
    pub fn deserialize_bytes(&mut self, bytes: Vec<u8>) {
        let mut stream_reader = StreamReader::new(bytes);
        self.deserialize(&mut stream_reader);
    }
}

impl Deserializable for GDPlayer {
    fn deserialize(&mut self, stream_reader: &mut StreamReader) {
        let x = stream_reader.read_f32();
        let y = stream_reader.read_f32();
        self.owner_id = stream_reader.read_u32();

        let new_position = Vector2::new(x, y);
        self.base_mut().set_position(new_position);
    }
}
