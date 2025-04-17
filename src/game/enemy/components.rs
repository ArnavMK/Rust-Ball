
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3
}

#[derive(Resource)]
pub struct EnemySpeed {
    pub speed: f32,
    pub original_speed: f32
}

impl Default for EnemySpeed {
    fn default() -> Self {
        Self {
            speed: 270.0,
            original_speed: 270.0
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(10.0, TimerMode::Repeating)
        }
    }
}


