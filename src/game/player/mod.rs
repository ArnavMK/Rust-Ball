use bevy::prelude::*;
use systems::*;
use events::*;


pub mod components;
pub mod events; 
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, spawn_player)
        .add_event::<OnPlayerCollisionStateChanged>()
        .add_systems(Update, (
                player_movement,
                player_boundry_checker
            ).chain())
        .add_systems(Update, player_collision)
        .add_systems(Update, toggle_player_collision)
        .add_systems(Update, toggle_player_collision_visual);

    }
}
