use crate::input::input_manager::InputManager;
use bevy::app::{App, Plugin};

pub mod input_manager;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputManager { server_frame: 0 });
    }
}
