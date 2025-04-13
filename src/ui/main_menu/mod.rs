use bevy::prelude::*;
use crate::AppState;
use systems::layout::*;
use systems::interactions::*;
use crate::ui::events::*;
use crate::game::score::systems::create_player_log_file;

pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {

        app

            // On Enter
            .add_systems(OnEnter(AppState::MainMenu), (
                create_player_log_file,
                spawn_main_menu
            ).chain())
            
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
