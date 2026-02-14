use godot::prelude::*;

mod message_header;
mod network_manager;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
