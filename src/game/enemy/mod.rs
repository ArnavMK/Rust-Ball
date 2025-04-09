use bevy::prelude::*;
use components::*;
use systems::*;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enimies)
        .add_systems(Update, (
                enemy_movement,
                enemy_confinement
            ).chain());
    }
}
