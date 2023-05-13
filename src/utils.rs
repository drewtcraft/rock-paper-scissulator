use bevy::prelude::*;
use rand::prelude::*;

pub const ENTITY_COUNT: u16 = 10;
pub const ENTITY_MAX_SPEED: f32 = 10.0;
pub const ENTITY_SIZE: f32 = 64.0;
pub const ENTITY_HALF_SIZE: f32 = 32.0;
pub const ENTITY_ACCELERATION: f32 = 1.0;
pub const TIME_FACTOR: f32 = 1.0;

pub fn generate_random_vec3_in_bounds(bounds: &Window) -> Vec3 {
    let random_x = random::<f32>() * bounds.width();
    let random_y = random::<f32>() * bounds.height();
    Vec3::new(random_x, random_y, 0.0)
}

pub fn generate_exclusive_transform(bounds: &Window, taken_positions: &mut Vec<Vec3>) -> Transform {
    let the_right_place = loop {
        let random_vec3 = generate_random_vec3_in_bounds(bounds);
        if !vec3_conflicts_with_existing(random_vec3.clone(), &taken_positions) {
            break random_vec3;
        }
    };

    taken_positions.push(the_right_place);

    Transform::from_xyz(the_right_place.x, the_right_place.y, 0.0)
}

pub fn vec3_conflicts_with_existing(vec3: Vec3, taken_positions: &Vec<Vec3>) -> bool {
    for position in taken_positions {
        if position.distance(vec3) < (ENTITY_SIZE + 5.0) {
            return true;
        }
    }
    false
}
