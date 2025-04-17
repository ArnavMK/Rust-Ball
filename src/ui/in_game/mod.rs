use bevy::prelude::*;
use crate::ui::in_game::systems::layout::*;
use crate::game::GameState;
use crate::AppState;
use super::in_game::systems::interactions::*;
use super::events::*;
pub mod systems;

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
        
            // Ui interactions
            .add_systems(Update, (
                resume_button_interaction,
                quit_button_interaction
            ).run_if(in_state(AppState::InGame).and_then(in_state(GameState::Paused))))
            
            // On exit
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu)

            .add_event::<OnResumeButtonClicked>()
        
            // Score UI
            .add_systems(OnEnter(AppState::InGame), spawn_score_ui)
            .add_systems(Update, update_score_ui.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
            .add_systems(OnExit(AppState::InGame), despawn_score_ui)

            // countdown UI
            .add_systems(OnEnter(AppState::InGame), spawn_countdown_ui)
            .add_systems(OnExit(GameState::Countdown), despawn_countdown_ui)

            // Fuel bar
            .add_systems(OnEnter(AppState::InGame), spawn_fuel_ui)
            .add_systems(Update, update_fuel_ui.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
            .add_systems(OnExit(AppState::InGame), despawn_fuel_ui)

            // Test powerup UI
            .add_systems(OnEnter(AppState::InGame), spawn_powerup_ui)
            .add_systems(Update, update_powerup_bar.run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
            .add_systems(OnExit(AppState::InGame), despawn_powerup_ui)
        ;
    }
}
