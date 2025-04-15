use bevy::prelude::*;
use systems::*;
use events::*;
use crate::game::GameState;
use crate::AppState;
use crate::game::player::resources::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerSpeed>()
            .init_resource::<PlayerTextures>()
            .init_resource::<ActivePowerups>()
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_plugins(PlayerEvents)
            .add_systems(Update, (
                player_movement,
                player_boundry_checker
            ).chain().run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))

            .add_systems(Update, (
                toggle_player_collision,
                toggle_player_collision_visual,
                player_collision,
            ).chain().run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))
            
            .add_systems(Update, (
                collect_powerups,
                apply_powerup,
                remove_powerup
            ).run_if(in_state(AppState::InGame).and_then(in_state(GameState::Running))))

            .add_systems(OnExit(AppState::InGame), (despawn_player, force_remove_powerups))
        ;
    }
}
