use bevy::prelude::*;
use systems::*;
use crate::game::GameState;
use crate::AppState;

mod systems;

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app 
            .init_resource::<CountdownTimer>()
            .add_systems(Update, update_countdown_ui.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Countdown))))
        ;
    }
}
