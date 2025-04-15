use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Powerup {
    
    SpeedBoost {
        multiplier: f32,
        duration: f32,
    },
    Freez {
        multiplier: f32,
        duration: f32
    }

}

impl Powerup {
    pub fn duration(&self) -> f32 {
        match self {
            Powerup::SpeedBoost {duration, ..} => *duration,
            Powerup::Freez {duration, ..} => *duration
        }
    }
}

pub const POWERUP_DEFINITIONS: &[(Powerup, &str)] = &[
    (Powerup::SpeedBoost {duration: 7.0, multiplier: 1.5}, "sprites/speed_boost.png"),
    (Powerup::Freez {multiplier: 2.0, duration: 5.0}, "sprites/test_power.png")
];
