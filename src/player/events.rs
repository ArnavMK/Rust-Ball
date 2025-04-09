use bevy::prelude::*;


#[derive(Event)]
pub struct OnPlayerCollisionStateChanged {
    pub state: bool
}
