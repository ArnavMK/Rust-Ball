
use bevy::prelude::*;
use crate::game::GameState;
use crate::ui::events::*;

pub fn toggle_state(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running),
            GameState::Countdown => {}
        }
    }
}

pub fn on_resume_button_cliked(
    mut next_state: ResMut<NextState<GameState>>,
    mut event: EventReader<OnResumeButtonClicked>,
) {
    for _ in event.read() {
        next_state.set(GameState::Running);
    }
}
