use crate::linking_context::Serializable;
use crate::network_manager::GDNetworkManager;
use common::serializer::Serializer;
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct GDPlayer {
    pub base: Base<CharacterBody2D>,

    network_manager: Option<Gd<GDNetworkManager>>,
}

impl Serializable for GDPlayer {
    fn serialize(&mut self) -> Vec<u8> {
        let mut serializer = Serializer::new(vec![]);

        let position = self.base().get_position();
        let _ = &mut serializer << position.x;
        let _ = &mut serializer << position.y;

        serializer.get_data().to_vec()
    }
    fn deserialize(&mut self, bytes: Vec<u8>) {
        let mut serializer = Serializer::new(bytes);

        let mut x = 0.0;
        let mut y = 0.0;

        let _ = &mut serializer >> &mut x;
        let _ = &mut serializer >> &mut y;

        let new_position = Vector2::new(x, y);
        self.base_mut().set_position(new_position);
    }
}

#[godot_api]
impl ICharacterBody2D for GDPlayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
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
