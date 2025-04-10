use bevy::prelude::*;
use crate::AppState;
use systems::layout::*;
use systems::interactions::*;
use events::*;

mod systems;
mod components;
mod styles;
pub mod events;

pub struct MainMenuScene;

impl Plugin for MainMenuScene {
    fn build(&self, app: &mut App) {

        app

            // On Enter
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            
            // On Exit
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)

            // UI interactions
            .add_systems(Update, (
                play_button_interaction,
                quit_button_interaction
            ).run_if(in_state(AppState::MainMenu)))
           
            // Events
            .add_event::<OnPlayButtonClicked>()
            .add_event::<OnQuitButtonClicked>()
        ;
    }
}
