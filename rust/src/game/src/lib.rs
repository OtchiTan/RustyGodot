use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

mod message_type;
mod network_manager;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Player initialized!");
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // Simple rotation logic
        let radians = self.angular_speed * delta;
        self.base_mut().rotate(radians as f32);

        // Simple movement logic
        let velocity = Vector2::UP.rotated(self.base().get_rotation()) * self.speed as f32;
        let change = velocity * delta as f32;
        self.base_mut().translate(change);
    }
}
