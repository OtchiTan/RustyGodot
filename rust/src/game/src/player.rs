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

    owner_id: u32,
    network_manager: Option<Gd<GDNetworkManager>>,
}

impl Serializable for GDPlayer {
    fn serialize(&mut self) -> Vec<u8> {
        let mut serializer = Serializer::new(vec![]);

        let position = self.base().get_position();
        let _ = &mut serializer << position.x;
        let _ = &mut serializer << position.y;
        let _ = &mut serializer << self.owner_id;

        serializer.get_data().to_vec()
    }
    fn deserialize(&mut self, bytes: Vec<u8>) {
        let mut serializer = Serializer::new(bytes);

        let mut x = 0.0;
        let mut y = 0.0;

        let _ = &mut serializer >> &mut x;
        let _ = &mut serializer >> &mut y;
        let _ = &mut serializer >> &mut self.owner_id;

        let new_position = Vector2::new(x, y);
        self.base_mut().set_position(new_position);
    }
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
        let mut is_locally_owned = false;
        let nm_node = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<GDNetworkManager>("%GDNetworkManager");

        if let Ok(nm) = nm_node.try_cast::<GDNetworkManager>() {
            let network_client_id = nm.bind().client_id;
            is_locally_owned = network_client_id == self.owner_id;
        }

        is_locally_owned
    }
}
