use bevy::prelude::*;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::in_game::InGameUiPlugin;
use crate::ui::game_over::GameOverUiPlugin;

pub mod main_menu;
pub mod events;
pub mod styles;
pub mod in_game;
pub mod game_over;
pub mod components;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((MainMenuPlugin, InGameUiPlugin, GameOverUiPlugin))
        ;
    }
}
