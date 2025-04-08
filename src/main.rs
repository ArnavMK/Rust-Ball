use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;
const NUMBER_OF_ENIMIES: u32 = 10;
const ENEMY_SPEED: f32 = 250.0;
const ENEMY_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enimies)
        .add_systems(Startup, spawn_player)
        .add_event::<OnPlayerCollisionStateChanged>()
        .add_systems(Update, player_movement)
        .add_systems(Update, player_boundry_checker)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, enemy_confinement)
        .add_systems(Update, player_collision)
        .add_systems(Update, toggle_player_collision)
        .add_systems(Update, testing)
    .run();
}

#[derive(Component)]
pub struct Player {
    can_collide: bool,
}

#[derive(Component)]
pub struct Enemy {
    direction: Vec3
}

#[derive(Event)]
pub struct OnPlayerCollisionStateChanged {
    state: bool
}

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
        }
    ));
}

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

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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


pub fn testing(
    mut event: EventReader<OnPlayerCollisionStateChanged>,
    mut player_texture: Query<(&Player, &mut Handle<Image>), With<Player>>,
    asset_server: Res<AssetServer>
) {
    for e in event.read() {
        
        if let Ok((player, mut texture)) = player_texture.get_single_mut() {
            
            *texture = if e.state {
                asset_server.load("sprites/ball_blue_small.png")
            }
            else {
                asset_server.load("sprites/hole.png")
            }

        }
    }
}
