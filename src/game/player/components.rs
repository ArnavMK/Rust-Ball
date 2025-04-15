use bevy::prelude::*;
use crate::game::power_ups::components::*;

#[derive(Clone)]
pub struct ActivePowerup {
    pub kind: Powerup,
    pub timer: Timer,
    pub power_applied: bool
}

#[derive(Component)]
pub struct Player {
    pub can_collide: bool,
    pub active_powerups: Vec<ActivePowerup>,
    pub collidable_texture: Handle<Image>,
    pub not_collidable_texture: Handle<Image>,
}


