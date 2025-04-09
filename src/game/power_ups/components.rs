use bevy::prelude::*;

pub enum PowerupType {
    Refule, // to resule the invisiblity of the player.
    Decelerator, // to slow down other enimies.
    Multiplier // to increase the increment of the score.
}

#[derive(Component)]
pub struct Powerup {
    power: PowerupType
}


