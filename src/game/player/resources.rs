use bevy::prelude::*;
use crate::game::power_ups::components::Powerup;

#[derive(Resource)]
pub struct PlayerSpeed {
    pub speed: f32,
    pub original_speed: f32
}

impl Default for PlayerSpeed {
    fn default() -> Self {
        Self {
            speed: 469.0,
            original_speed: 469.0
        }
    }
}

#[derive(Resource)]
pub struct PlayerTextures {
    pub collidable: Handle<Image>,
    pub not_collidable: Handle<Image>
}

impl FromWorld for PlayerTextures {

    fn from_world(world: &mut World) -> Self {

        let asset_server = world.resource::<AssetServer>();
        Self {
            collidable: asset_server.load("sprites/ball_blue_small.png"),
            not_collidable: asset_server.load("sprites/hole.png")
        }

    }
}

#[derive(Resource)]
pub struct ActivePowerups {
    pub powerups: Vec<(Powerup, Entity, Timer)>
}

impl Default for ActivePowerups {
    fn default() -> Self {
        Self {
            powerups: Vec::new()        
        }
    }
}

#[derive(Resource)]
pub struct Fuel {
    pub amount: f32,
    pub decrease_speed: f32,
    pub increase_speed: f32,
    pub empty: bool
}

impl Default for Fuel {
    fn default() -> Self {
        Self {
            amount: 100.0, 
            decrease_speed: 80.0,
            increase_speed: 20.0,
            empty: false
        }
    }
}
