use bevy::prelude::*;
use crate::AppState;
use crate::ui::main_menu::systems::interactions::*;
use layout::*;

pub mod layout;


pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), spawn_final_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_final_menu)

            .add_systems(Update, (
                play_button_interaction,
                quit_button_interaction
             ).run_if(in_state(AppState::GameOver)))
        ;
    }
}
