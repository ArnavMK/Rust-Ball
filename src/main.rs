use bevy::prelude::*;
use crate::game::Game;
use crate::main_menu::*;

pub mod main_menu;
pub mod game;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_plugins((Game, MainMenuScene))
    .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver
}

