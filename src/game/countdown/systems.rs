use bevy::prelude::*;
use crate::ui::components::CountdownText;
use crate::game::GameState;

#[derive(Resource)]
pub struct CountdownTimer {
    pub timer: Timer,
    pub max_ticks: u32,
    pub max_ticks_buffer: u32
}

impl Default for CountdownTimer {
    fn default() -> CountdownTimer {
        CountdownTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            max_ticks: 3, max_ticks_buffer: 3
        }
    }
}

pub fn update_countdown_ui(
    mut countdown: ResMut<CountdownTimer>,
    time: Res<Time>,
    mut countdown_text_query: Query<&mut Text, With<CountdownText>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if countdown.timer.tick(time.delta()).finished() { 

        if countdown.max_ticks >= 1 {
            if let Ok(mut countdown_text) = countdown_text_query.get_single_mut() {
                countdown_text.sections[0].value = format!("{}", countdown.max_ticks);
                countdown.max_ticks -= 1;
            }
        }
        else {
            println!("Changing the game state");
            next_state.set(GameState::Running);
            countdown.max_ticks = countdown.max_ticks_buffer;
        }
    }
}

