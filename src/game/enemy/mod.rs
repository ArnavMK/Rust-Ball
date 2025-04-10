use bevy::prelude::*;
use components::*;
use systems::*;
use crate::game::GameState;
use crate::AppState;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_enimies)
        .add_systems(Update, (
                enemy_movement,
                enemy_confinement
        )
        .chain()
        .run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))));
    }
}
