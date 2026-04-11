use crate::input_manager::GDInputManager;
use crate::network_manager::GDNetworkManager;
use common::stream_reader::StreamReader;
use godot::builtin::math::FloatExt;
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::obj::Singleton;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct GDPlayer {
    pub base: Base<CharacterBody2D>,

    owner_id: u32,
    network_manager: Option<Gd<GDNetworkManager>>,
    #[export]
    input_manager: Option<Gd<GDInputManager>>,
}

#[godot_api]
impl ICharacterBody2D for GDPlayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            owner_id: 0,
            network_manager: None,
            input_manager: None,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.is_locally_owned() {
            let input = Input::singleton();
            let direction = input.get_vector("move_left", "move_right", "move_down", "move_up");

            if let Some(input_manager) = &mut self.input_manager {
                input_manager.bind_mut().add_direction_input(direction);
            }

            self.base_mut().set_velocity(direction * 500.0);
            self.base_mut().move_and_slide();
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

        let velocity = sr2.read_vec2();

        self.owner_id = sr2.read_u32();

        let mut x = position2.x;
        let mut y = position2.y;

        if !self.is_locally_owned() {
            x = position1.x.lerp(position2.x, alpha);
            y = position1.y.lerp(position2.y, alpha);

            self.base_mut()
                .set_velocity(Vector2::new(velocity.x, velocity.y));
        }

        self.base_mut().set_position(Vector2::new(x, y));
    }
}
