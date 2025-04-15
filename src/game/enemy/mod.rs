use bevy::prelude::*;
use systems::*;
use crate::game::GameState;
use crate::AppState;
use components::EnemySpeed;

pub mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {

    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpeed>()
        .add_systems(OnEnter(AppState::InGame), spawn_enimies)
        .add_systems(Update, handle_enemy_freez.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
        .add_systems(Update, (
                enemy_confinement,
                enemy_movement,
        )
        .chain()
        .run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
        
        .add_systems(OnExit(AppState::InGame), (despawn_enemies, force_reset_speed))
        ;
    }
}
