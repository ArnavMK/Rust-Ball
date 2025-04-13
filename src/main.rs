use bevy::prelude::*;
use crate::game::*;
use crate::ui::events::*;
use crate::game::player::events::*;
use crate::ui::UiPlugin;

pub mod game;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (
            game_over_on_player_death.run_if(in_state(AppState::InGame)),
            start_game_state.run_if(in_state(AppState::MainMenu).or_else(in_state(AppState::GameOver))),
        ))
        .add_plugins((Game, UiPlugin))

    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver
}

fn game_over_on_player_death(
    mut next_state: ResMut<NextState<AppState>>,
    mut event: EventReader<OnPlayerDeath>,
) {
    for _ in event.read() {
        next_state.set(AppState::GameOver);
    }
}


fn start_game_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut event: EventReader<OnPlayButtonClicked>
) {
    for _ in event.read() {
        next_app_state.set(AppState::InGame);
        next_game_state.set(GameState::Countdown);
    }
}
