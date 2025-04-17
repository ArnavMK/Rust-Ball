use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct FinalMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct ScoreDisplayText;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct CountdownMenu;

#[derive(Component)]
pub struct CountdownText;

#[derive(Component)]
pub struct FuelUi;


#[derive(Component)]
pub struct FuelBar;

#[derive(Component)]
pub struct PowerupBar;

#[derive(Component)]
pub struct PowerupSymbol {
    pub time_remaining: f32,
    pub total_time: f32
}

#[derive(Component)]
pub struct PowerupTimerBar;
