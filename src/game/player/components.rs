use bevy::prelude::*;


#[derive(Component)]
pub struct Player {
    pub can_collide: bool,
    pub collidable_texture: Handle<Image>,
    pub not_collidable_texture: Handle<Image>,
}


