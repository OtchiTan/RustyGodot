use godot::prelude::*;

mod message_header;
mod network_manager;
mod player;
mod serializer;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

trait Serializable {
    fn serialize(&self, buffer: &mut [u8]) -> usize;
    fn deserialize(&mut self, buffer: &[u8]) -> Self;
}