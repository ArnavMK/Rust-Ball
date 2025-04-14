use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum Powerup {
    
    SpeedBoost {
        multiplier: u32,
        duration: f32,
    },
    TestPower {
        text: u32
    }
}

pub const POWERUP_DEFINITIONS: &[(Powerup, &str)] = &[
    (Powerup::SpeedBoost {duration: 10.3, multiplier: 10}, "sprites/speed_boost.png"),
    (Powerup::TestPower {text: 212}, "sprites/test_power.png")
];
