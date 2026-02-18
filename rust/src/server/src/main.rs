mod network;
mod replication;
mod rpc;

use crate::network::NetworkPlugin;
use crate::replication::ReplicationPlugin;
use crate::rpc::RpcPlugin;
use bevy::MinimalPlugins;
use bevy::app::App;

const SERVER_IP: &str = "127.0.0.1:3630";

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(NetworkPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(RpcPlugin)
        .run();
}
