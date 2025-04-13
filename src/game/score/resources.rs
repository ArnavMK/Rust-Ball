use bevy::prelude::*;
use super::SCORE_INCREMENT_TIME;

#[derive(Resource)]
pub struct ScoreIncrementTimer {
    pub timer: Timer
}

impl Default for ScoreIncrementTimer {
    fn default() -> ScoreIncrementTimer {
        ScoreIncrementTimer {
            timer: Timer::from_seconds(SCORE_INCREMENT_TIME, TimerMode::Repeating)
        }
    } 
}

#[derive(Resource)]
pub struct Score {  
    pub value: u32,
    pub score_delta: u32,
    pub highest_score: u32
}

impl Default for Score {
    fn default() -> Score {
        Score {
            value: 0,
            score_delta: 10,
            highest_score: 0
        }
    }
}

