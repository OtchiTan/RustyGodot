use godot::classes::{ISprite2D, Sprite2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct GDPlayer {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for GDPlayer {
    fn init(base: Base<Sprite2D>) -> Self {
        Self { base }
    }
}
