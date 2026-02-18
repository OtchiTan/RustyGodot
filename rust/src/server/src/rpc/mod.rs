use crate::rpc::rpc_manager::RpcManager;
use bevy::app::{App, Plugin};

pub mod rpc_manager;

pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RpcManager {});
    }
}
