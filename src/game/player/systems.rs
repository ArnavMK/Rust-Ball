use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::*;
use crate::game::enemy::components::*;
use crate::game::power_ups::components::Powerup;
use super::resources::*;
use super::events::*;

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
            active_powerups: Vec::new(),
            collidable_texture: asset_server.load("sprites/ball_blue_small.png"),
            not_collidable_texture: asset_server.load("sprites/hole.png")
        }
    ));
}


pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    player_speed: Res<PlayerSpeed>
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
            transform.translation += direction * player_speed.speed * time.delta_seconds();
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
    mut commands: Commands,
    mut event: EventWriter<OnPlayerDeath>
) {

    if let Ok((player_entity, player_transform, player)) = player_query.get_single() {

        if !player.can_collide {return;}

        let collision_threshold: f32 = 64.0;
        for transform in &enemy_query {

            let distance = transform.translation.distance(player_transform.translation);
            if distance <= collision_threshold {
                event.send(OnPlayerDeath);
                commands.entity(player_entity).despawn();
            }

        }
    }
}

pub fn collect_powerups(
    player_query: Query<(&Transform, &Player), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &Powerup)>,
    mut commands: Commands,
    mut events: EventWriter<PowerupCollected>
) {

    if let Ok((player_transform, player)) = player_query.get_single() {

        if !player.can_collide {return;}

        for (entity, transform, powerup) in &powerup_query {

            let distance = transform.translation.distance(player_transform.translation);
            
            let distance_threshold = PLAYER_SIZE/2.0 + 32.0;
            if distance <= distance_threshold {

                events.send(PowerupCollected {
                    powerup: powerup.clone(), entity
                });
                
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn apply_powerup(
    mut active_powerups: ResMut<ActivePowerups>,
    mut powerup_collected_event: EventReader<PowerupCollected>,
    mut powerup_expired_events: EventWriter<PowerupExpired>,
    mut freez_enemy_event: EventWriter<FreezEnemyEvent>,
    time: Res<Time>,
    mut player_speed: ResMut<PlayerSpeed>
) {

    for event in powerup_collected_event.read() {
        
        let powerup = event.powerup.clone();
        
        active_powerups.powerups.push((
            powerup,
            event.entity,
            Timer::from_seconds(event.powerup.duration(), TimerMode::Once)
        ));

       match powerup {
            Powerup::SpeedBoost {multiplier, ..} => {
                player_speed.original_speed = player_speed.speed;
                player_speed.speed *= multiplier;
                println!("SpeedBoost applied: {}m/s", player_speed.speed);
            }   
            Powerup::Freez {multiplier, ..} => {
                freez_enemy_event.send(FreezEnemyEvent {multiplier, being_applied: true});
            }
        }
    }

    active_powerups.powerups.retain_mut(|(powerup, entity, timer)| {
        timer.tick(time.delta());

        if timer.finished() {
            powerup_expired_events.send(PowerupExpired {
                powerup: powerup.clone(),
                entity: *entity
            });
            return false;
        }
        true
    });

}

pub fn remove_powerup(
    mut player_speed: ResMut<PlayerSpeed>,
    mut powerup_expired_events: EventReader<PowerupExpired>,
    mut freez_enemy_event: EventWriter<FreezEnemyEvent>
) {

    for e in powerup_expired_events.read() {

        match e.powerup {
            Powerup::SpeedBoost {..} => {
                player_speed.speed = player_speed.original_speed;
                println!("SpeedBoost removed: {}m/s", player_speed.speed);
            },
            Powerup::Freez {multiplier, ..} => {
                freez_enemy_event.send(FreezEnemyEvent {multiplier, being_applied: false});
            }
        }
    }
}


pub fn force_remove_powerups(
    mut player_speed: ResMut<PlayerSpeed>,
    mut freez_enemy_event: EventWriter<FreezEnemyEvent>,
    active_powerups: Res<ActivePowerups>
) {
    
    for (powerup, ..) in active_powerups.powerups.iter() {
        println!("FORCE REMOVING");
        match powerup {
            Powerup::SpeedBoost {..} => {
                player_speed.speed = player_speed.original_speed;
            },
            Powerup::Freez {multiplier, ..} => {
                freez_enemy_event.send(FreezEnemyEvent {multiplier: *multiplier, being_applied: false});
            }
        }
    }

}

pub fn toggle_player_collision(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut event: EventWriter<OnPlayerCollisionStateChanged>,
    fuel: Res<Fuel>
) {
            
    if let Ok(mut player) = player_query.get_single_mut() {

        if !fuel.empty {
            
            // start immunity
            if input.just_pressed(KeyCode::Space) && player.can_collide {
                player.can_collide = false;
                event.send(OnPlayerCollisionStateChanged {state: false});
            }
        
            // disable immunity
            if input.just_released(KeyCode::Space) && !player.can_collide {
                player.can_collide = true;
                event.send(OnPlayerCollisionStateChanged {state: true});
            }
        }
        else {
            if !player.can_collide {
                player.can_collide = true;
                event.send(OnPlayerCollisionStateChanged {state: true});
            }
        }
    }
}

pub fn fuel_system(
    mut fuel: ResMut<Fuel>,
    player_query: Query<&Player>,
    time: Res<Time>
) {
    if let Ok(player) = player_query.get_single() {
    
        let mut amount = fuel.amount;

        if !player.can_collide && !fuel.empty {
            amount -= fuel.decrease_speed * time.delta_seconds();
        }

        fuel.amount = amount.clamp(0.0, 100.0);

        if fuel.amount == 0.0 && !fuel.empty{
            fuel.empty = true;
        }

        if fuel.empty {
            amount += fuel.increase_speed * time.delta_seconds();
        }

        fuel.amount = amount.clamp(0.0, 100.0);

        if fuel.amount == 100.0 && fuel.empty {
            fuel.empty = false;
        }
    }
}

pub fn reset_fuel_system(mut fuel: ResMut<Fuel>) {
    fuel.empty = false;
    fuel.amount = 100.0;
}

pub fn toggle_player_collision_visual(
    mut event: EventReader<OnPlayerCollisionStateChanged>,
    mut player_query: Query<&mut Handle<Image>, With<Player>>,
    player_textures: Res<PlayerTextures>
) {
    for e in event.read() {
        
        if let Ok(mut texture) = player_query.get_single_mut() {
            
            *texture = if e.state {
                player_textures.collidable.clone_weak()
            }
            else {
                player_textures.not_collidable.clone_weak()
            }
        }
    }
}

pub fn despawn_player(
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}
