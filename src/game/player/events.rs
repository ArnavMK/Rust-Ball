use bevy::prelude::*;
use crate::game::power_ups::components::Powerup;


#[derive(Event)]
pub struct OnPlayerCollisionStateChanged {
    pub state: bool
}

#[derive(Event)]
pub struct PowerupCollected {
    pub powerup: Powerup,
    pub entity: Entity
}

#[derive(Event)]
pub struct PowerupExpired {
    pub powerup: Powerup,
    pub entity: Entity
}

#[derive(Event)]
pub struct FreezEnemyEvent {
    pub multiplier: f32,
    pub being_applied: bool
}

#[derive(Event)]
pub struct OnPlayerDeath;

pub struct PlayerEvents; 

impl Plugin for PlayerEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPlayerCollisionStateChanged>()
        .add_event::<OnPlayerDeath>()
        .add_event::<PowerupCollected>()
        .add_event::<PowerupExpired>()
        .add_event::<FreezEnemyEvent>()
        ;
    }
}
