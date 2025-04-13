use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use super::components::*;

const NUMBER_OF_ENIMIES: u32 = 15;
const ENEMY_SPEED: f32 = 300.0;
const ENEMY_SIZE: f32 = 64.0;


pub fn spawn_enimies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer> 
) {

    if let Ok(window) = window_query.get_single() {

        for _ in 0..NUMBER_OF_ENIMIES {

            let x = (random::<f32>() * (window.width() - ENEMY_SIZE * 2.0)) - (window.width()/2.0 - ENEMY_SIZE);
            let y = (random::<f32>() * (window.height() - ENEMY_SIZE * 2.0)) - (window.height()/2.0 - ENEMY_SIZE);   

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_small.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec3::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0, 0.0).normalize()
                }
            ));
        }
    }
}

pub fn enemy_movement(
    mut enemy_transform: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in &mut enemy_transform {
        transform.translation += ENEMY_SPEED * enemy.direction * time.delta_seconds();
    }
}


pub fn enemy_confinement(
    mut enemy_transform: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(window) = window_query.get_single() {
        
        let radius = ENEMY_SIZE / 2.0;
        let left_egde = -window.width() / 2.0;
        let right_egde = -left_egde;
        let upper_egde = window.height() / 2.0;
        let lower_egde = -upper_egde;
    
        for (transform, mut enemy) in &mut enemy_transform {
            if transform.translation.x <= left_egde + radius || transform.translation.x >= right_egde - radius {enemy.direction.x *= -1.0;};
            if transform.translation.y <= lower_egde || transform.translation.y >= upper_egde - radius {enemy.direction.y *= -1.0};
        }

    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}
