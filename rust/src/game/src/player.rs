use crate::network_manager::GDNetworkManager;
use common::stream_reader::StreamReader;
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
impl GDPlayer {
    #[func]
    pub fn is_locally_owned(&self) -> bool {
        if let Some(network_manager) = &self.network_manager {
            return self.owner_id == network_manager.bind().client_id;
        }
        false
    }

    #[func]
    pub fn deserialize_bytes(&mut self, bytes: Vec<u8>) {
        let mut stream_reader = StreamReader::new(bytes);
        let x = stream_reader.read_f32();
        let y = stream_reader.read_f32();
        self.owner_id = stream_reader.read_u32();

        let new_position = Vector2::new(x, y);
        self.base_mut().set_position(new_position);
    }
}
