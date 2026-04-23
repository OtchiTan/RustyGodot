use crate::network_manager::GDNetworkManager;
use common::stream_reader::StreamReader;
use godot::builtin::math::FloatExt;
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
    pub fn deserialize_bytes(&mut self, snap1: Vec<u8>, snap2: Vec<u8>, alpha: f32) {
        let mut sr1 = StreamReader::new(snap1);
        let mut sr2 = StreamReader::new(snap2);

        let position1 = sr1.read_vec2();
        let position2 = sr2.read_vec2();
        self.owner_id = sr2.read_u32();

        let old_position = Vector2::new(position1.x, position1.y);
        let mut next_position = Vector2::new(position2.x, position2.y);

        if !self.is_locally_owned() {
            next_position = old_position.lerp(next_position, alpha);

            self.base_mut().set_position(next_position);
            return;
        }

        if self.base().get_position().distance_to(next_position) >= 150.0 {
            self.base_mut().set_position(next_position);
        }
    }
}
