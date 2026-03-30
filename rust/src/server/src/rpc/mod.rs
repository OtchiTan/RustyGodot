use crate::rpc::rpc_manager::handle_input;
use bevy::app::{App, Plugin};

pub mod rpc_manager;

pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_input);
    }
}
