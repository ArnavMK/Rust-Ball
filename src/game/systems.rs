use bevy::prelude::*;
use crate::game::GameState;

pub fn toggle_state(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>
) {
    if input.pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running)
        }
    }
}
