use crate::network_manager::GDNetworkManager;
use common::stream_reader::StreamReader;
use godot::builtin::Vector2;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::global::{exp};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

const MAX_HARD_SNAP: f64 = 150.0;
const ERROR_DISTANCE: f64 = 10.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct GDPlayer {
    pub base: Base<CharacterBody2D>,

    owner_id: u32,
    network_manager: Option<Gd<GDNetworkManager>>,

    #[var]
    replicated_velocity: Vector2,
}

#[godot_api]
impl ICharacterBody2D for GDPlayer {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            owner_id: 0,
            network_manager: None,
            replicated_velocity: Vector2::new(0.0, 0.0),
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

        let old_position = Vector2::new(position1.x, position1.y);
        let mut next_position = Vector2::new(position2.x, position2.y);

        if !self.is_locally_owned() {
            next_position = old_position.lerp(next_position, alpha);
            self.replicated_velocity = Vector2::new(velocity.x, velocity.y);
            self.base_mut().set_position(next_position);
            return;
        }

        let current_pos = self.base().get_position();
        let error_vec = next_position - current_pos;
        let dist_error = error_vec.length() as f64;

        if dist_error <= ERROR_DISTANCE {
            return;
        }

        if dist_error >= MAX_HARD_SNAP {
            self.base_mut().set_position(next_position);
            return;
        }

        let delta_time = self.base().get_process_delta_time();

        // Bon bas là ça applique la méthode donné en cours, j'ai rien compris mais ça à l'air de marcher
        // Par contre j'ai du baisser les paramètres de fou par apport aux exemples donnés, aucune idée de pourquoi

        let g = 5.0;
        let k = 15.0;
        let d = 0.5;

        let urgency = g * (1.0 / (MAX_HARD_SNAP - ERROR_DISTANCE));

        let exponent = exp(d * dist_error * delta_time);
        let correction_mag = urgency * (k * dist_error * delta_time / exponent);

        let correction_vector = error_vec.normalized() * (correction_mag as f32);

        let new_pos = current_pos + correction_vector;
        self.base_mut().set_position(new_pos);

        // Bon en vrai j'ai compris l'idée générale, mais les math et la physique j'ai trop de lacunne pour l'expliquer
    }
}
