use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use super::components::*;
use crate::game::enemy::components::*;
use super::events::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

    let origin: Transform = Transform::from_xyz(0.0, 0.0, 1.0);

    commands.spawn((
        SpriteBundle {
            transform: origin,
            texture: asset_server.load("sprites/ball_blue_small.png"),
            ..default()
        },
        Player {
            can_collide: true,
            collidable_texture: asset_server.load("sprites/ball_blue_small.png"),
            not_collidable_texture: asset_server.load("sprites/hole.png")
        }
    ));
}


pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {

    if let Ok(mut transform) = player_transform.get_single_mut() {
            
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction.y += 1.0; 
        }

        if input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0; 
        }

        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn player_boundry_checker(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_transform: Query<&mut Transform, With<Player>>
) {

    if let Ok(window) = window_query.get_single() {
        
        let half_height = window.height()/2.0; 
        let half_width = window.width()/2.0;

        if let Ok(mut transform) = player_transform.get_single_mut() {

           let offset = PLAYER_SIZE / 2.0;

            transform.translation.x = transform.translation.x.clamp(-half_width + offset, half_width - offset); 
            transform.translation.y = transform.translation.y.clamp(-half_height+ offset, half_height - offset); 
        }
    }
}

pub fn player_collision(
    player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut commands: Commands
) {

    if let Ok((player_entity, player_transform, player)) = player_query.get_single() {

        if !player.can_collide {return;}

        let collision_threshold: f32 = PLAYER_SIZE;
        
        for transform in &enemy_query {
            let distance = f32::sqrt(
                (player_transform.translation.x - transform.translation.x).powi(2) +
                (player_transform.translation.y - transform.translation.y).powi(2)
            );
            
            if distance <= collision_threshold {
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn toggle_player_collision(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut event: EventWriter<OnPlayerCollisionStateChanged>
) {
            
    if let Ok(mut player) = player_query.get_single_mut() {

        if input.just_pressed(KeyCode::Space) && player.can_collide {
            player.can_collide = false;
            event.send(OnPlayerCollisionStateChanged {state: false});
        }

        if input.just_released(KeyCode::Space) && !player.can_collide {
            player.can_collide = true;
            event.send(OnPlayerCollisionStateChanged {state: true});
        }
    }
}


pub fn toggle_player_collision_visual(
    mut event: EventReader<OnPlayerCollisionStateChanged>,
    mut player_texture: Query<(&Player, &mut Handle<Image>), With<Player>>,
) {
    for e in event.read() {
        
        if let Ok((player, mut texture)) = player_texture.get_single_mut() {
            
            *texture = if e.state {
                player.collidable_texture.clone()
            }
            else {
                player.not_collidable_texture.clone()
            }

        }
    }
}
