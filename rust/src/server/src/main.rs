mod input;
mod network;
mod replication;

use crate::input::InputPlugin;
use crate::network::NetworkPlugin;
use crate::replication::ReplicationPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, Update};
use bevy::prelude::Query;
use bevy_rapier2d::prelude::*;

const SERVER_IP: &str = "127.0.0.1:3630";
const SERVER_FREQUENCY: f64 = 30.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(NetworkPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(InputPlugin)
        .add_systems(Update, update)
        .run();
}

fn update(velocities: Query<&Velocity>) {
    for velocity in velocities.iter() {
        println!("Velocity: {:?}", velocity);
    }
}
