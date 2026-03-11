use godot::prelude::*;

mod network_manager;
mod player;
mod linking_context;
mod replicated_node;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
