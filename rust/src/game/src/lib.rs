use godot::prelude::*;

mod network_manager;
mod player;
mod linking_context;
mod replicated_node;
mod input_manager;
mod stream_reader;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
