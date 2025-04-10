use bevy::prelude::*;


#[derive(Event)]
pub struct OnPlayerCollisionStateChanged {
    pub state: bool
}

#[derive(Event)]
pub struct OnPlayerDeath;

pub struct PlayerEvents; 

impl Plugin for PlayerEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPlayerCollisionStateChanged>()
        .add_event::<OnPlayerDeath>();
    }
}
