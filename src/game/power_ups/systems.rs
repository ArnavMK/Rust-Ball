use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use super::components::*;
use super::resources::*;

const POWERUP_SIZE: f32 = 32.0;

pub fn spawn_power_ups(
    mut commands: Commands,
    mut spawn_timer: ResMut<PowerUpSpawnTimer>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>
) {
       
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.finished() {

        if let Ok(window) = window_query.get_single() {
           
            let x = (random::<f32>() * (window.width() - POWERUP_SIZE * 2.0)) - (window.width()/2.0 - POWERUP_SIZE);
            let y = (random::<f32>() * (window.height() - POWERUP_SIZE * 2.0)) - (window.height()/2.0 - POWERUP_SIZE);   
   
            let (powerup, texture) = random_powerup();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 1.0),
                    texture: asset_server.load(*texture),
                    ..default()
                },
                powerup.clone()
            ));
        }
    }
}

fn random_powerup() -> &'static (Powerup, &'static str) {    
    
    let count = POWERUP_DEFINITIONS.len();
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..count);
    
    &POWERUP_DEFINITIONS[random_index]
}

pub fn despawn_powerups(
    mut commands: Commands,
    powerup_query: Query<Entity, With<Powerup>>
) {
    for powerup in &powerup_query {
        commands.entity(powerup).despawn();
    }
}
