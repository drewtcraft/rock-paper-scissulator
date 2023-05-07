use bevy::prelude::*;
use rand::prelude::*;

pub const ENTITY_COUNT: u16 = 5;
pub const ENTITY_MAX_SPEED: f32 = 100.0;
pub const ENTITY_SIZE: f32 = 64.0;
pub const ENTITY_HALF_SIZE: f32 = 32.0;
pub const ENTITY_ACCELERATION: f32 = 1.0;
pub const TIME_FACTOR: f32 = 2.0;

pub fn generate_random_transform(window: &Window) -> Transform {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    Transform::from_xyz(random_x, random_y, 0.0)
}
