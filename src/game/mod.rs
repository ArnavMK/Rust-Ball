use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use systems::*;
use crate::AppState;

pub mod enemy;
pub mod player;
pub mod power_ups;
mod systems;

pub struct Game;

impl Plugin for Game {

    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins((EnemyPlugin, PlayerPlugin))
            .add_systems(Update, toggle_state.run_if(in_state(AppState::InGame)));
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused
}
