use crate::player::GDPlayer;
use godot::classes::{Node, PackedScene};
use godot::obj::{Gd, GodotClass, Inherits};
use godot::tools::load;

pub trait Serializable {
    fn serialize(&mut self) -> Vec<u8>;
    fn deserialize(&mut self, bytes: Vec<u8>);
}

pub struct LinkingContext {
    spawn_functions: Vec<Box<dyn Fn() -> Gd<Node>>>,
}

impl LinkingContext {
    pub fn new() -> Self {
        let mut spawn_functions: Vec<Box<dyn Fn() -> Gd<Node>>> = Vec::new();

        Self::register::<GDPlayer>("scenes/player.tscn", &mut spawn_functions);

        Self { spawn_functions }
    }

    fn register<T>(scene_path: &str, spawn_functions: &mut Vec<Box<dyn Fn() -> Gd<Node>>>)
    where
        T: Inherits<Node> + GodotClass + Serializable,
    {
        let path = scene_path.to_string();
        spawn_functions.push(Box::new(move || {
            let scene: Gd<PackedScene> = load(&path);
            let instance = scene.instantiate_as::<Node>();
            instance
        }));
    }

    pub fn spawn(&self, type_id: usize) -> Gd<Node> {
        self.spawn_functions[type_id]()
    }

    pub fn serialize(&self, node: Gd<Node>) -> Vec<u8> {
        let mut data = vec![];
        if let Ok(mut node) = node.try_cast::<GDPlayer>() {
            data = node.bind_mut().serialize();
        }

        data
    }

    pub fn deserialize(&self, node: Gd<Node>, data: Vec<u8>) {
        if let Ok(mut node) = node.try_cast::<GDPlayer>() {
            node.bind_mut().deserialize(data);
        }
    }
}
