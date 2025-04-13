use bevy::prelude::*;
use resources::*;
use crate::game::GameState;
use crate::AppState;
use systems::*;

pub mod systems;
pub mod resources;

pub const SCORE_INCREMENT_TIME: f32 = 1.5;

pub struct ScoreSystemPlugin;

impl Plugin for ScoreSystemPlugin {
    fn build(&self, app: &mut App){
        app
            .init_resource::<ScoreIncrementTimer>()
            .init_resource::<Score>()
            .add_systems(Startup, create_player_log_file)
            .add_systems(Update, update_score_increment_timer)
            .add_systems(Update, increase_score.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
            .add_systems(Update, refresh_score_on_player_death)
            .add_systems(OnExit(AppState::InGame), save_score_in_file)
        ;
    }
}
