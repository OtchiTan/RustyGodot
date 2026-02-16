use crate::network_manager::GDNetworkManager;
use godot::classes::{ISprite2D, Sprite2D};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct GDPlayer {
    base: Base<Sprite2D>,
    pub net_id: u32,

    network_manager: Option<Gd<GDNetworkManager>>,
}

#[godot_api]
impl ISprite2D for GDPlayer {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            network_manager: None,
            net_id: 0,
        }
    }

    fn process(&mut self, _delta: f64) {
        if let Some(_network_manager) = &self.network_manager {

        }
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