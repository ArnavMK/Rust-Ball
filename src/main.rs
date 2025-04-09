use bevy::prelude::*;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;


pub mod enemy;
pub mod player;
pub mod power_ups;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_plugins((EnemyPlugin, PlayerPlugin))
    .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


