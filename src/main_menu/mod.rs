use bevy::prelude::*;


pub struct MainMenuScene;

impl Plugin for MainMenuScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start);
    }
}

pub fn start() {
    println!("You are on the main menue");
}
