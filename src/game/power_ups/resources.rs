use bevy::prelude::*;

#[derive(Resource)]
pub struct PowerUpSpawnTimer {
    pub timer: Timer
}

impl Default for PowerUpSpawnTimer {

    fn default() -> PowerUpSpawnTimer {
        PowerUpSpawnTimer {
            timer: Timer::from_seconds(13.0, TimerMode::Repeating)
        }
    }

}


