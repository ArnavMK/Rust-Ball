use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScoreSystemPlugin;
use countdown::CountdownPlugin;
use power_ups::PowerupPlugin;
use systems::*;
use crate::AppState;

pub mod enemy;
pub mod player;
pub mod power_ups;
pub mod score;
pub mod countdown;
mod systems;

pub struct Game;

impl Plugin for Game {

    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins((EnemyPlugin, PlayerPlugin, ScoreSystemPlugin, CountdownPlugin, PowerupPlugin))
            .add_systems(Update, (
                toggle_state.run_if(in_state(AppState::InGame)),
                on_resume_button_cliked.run_if(in_state(GameState::Paused))
            ));
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Countdown,
    Running,
    Paused
}
