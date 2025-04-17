use bevy::prelude::*;
use systems::*;
use crate::AppState;
use crate::game::GameState;
use resources::PowerUpSpawnTimer;

pub mod components; 
mod systems;
pub mod resources;


pub struct PowerupPlugin;

impl Plugin for PowerupPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PowerUpSpawnTimer>()
            .add_systems(Update, 
                spawn_power_ups.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running)))
            )
            .add_systems(OnExit(AppState::InGame), despawn_powerups)
        ;
    }
}
