use godot::prelude::*;

mod network_manager;
mod player;
mod linking_context;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
