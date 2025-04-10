use bevy::prelude::*;
use crate::game::Game;
use crate::main_menu::*;
use crate::game::player::events::*;
use crate::main_menu::events::*;

pub mod main_menu;
pub mod game;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            game_over_on_player_death.run_if(in_state(AppState::InGame)),
            start_game_state.run_if(in_state(AppState::MainMenu)),
            test.run_if(in_state(AppState::GameOver))
        ))
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

pub fn game_over_on_player_death(
    mut next_state: ResMut<NextState<AppState>>,
    mut event: EventReader<OnPlayerDeath>,
) {
    for _ in event.read() {
        next_state.set(AppState::GameOver);
    }
}

pub fn start_game_state(
    mut next_state: ResMut<NextState<AppState>>,
    mut event: EventReader<OnPlayButtonClicked>
) {
    for _ in event.read() {
        next_state.set(AppState::InGame);
    }
}

pub fn test() {
    println!("Game Over");
}
