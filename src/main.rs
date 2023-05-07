use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_startup_system(spawn_camera_ssys)
        .add_startup_system(spawn_entities_ssys)

        .add_system(movement_sys::<Rock, Paper, Scissors>)
        .add_system(movement_sys::<Paper, Scissors, Rock>)
        .add_system(movement_sys::<Scissors, Rock, Paper>)
        .add_system(contain_entities_sys)
        .add_system(detect_collisions_from_predators_sys::<Rock, Paper>)
        .add_system(detect_collisions_from_predators_sys::<Paper, Scissors>)
        .add_system(detect_collisions_from_predators_sys::<Scissors, Rock>)

        .run();
}

pub const ENTITY_COUNT: u16 = 20;
pub const ENTITY_SPEED: f32 = 100.0;
pub const ENTITY_SIZE: f32 = 64.0;
pub const ENTITY_HALF_SIZE: f32 = 32.0;

pub trait AssociatedString {
    const STRING: &'static str;
}

#[derive(Component, Default)]
pub struct Rock;

impl AssociatedString for Rock {
    const STRING: &'static str = "rock";
}

#[derive(Component, Default)]
pub struct Paper;

impl AssociatedString for Paper {
    const STRING: &'static str = "paper";
}

#[derive(Component, Default)]
pub struct Scissors;

impl AssociatedString for Scissors {
    const STRING: &'static str = "scissors";
}

// all Rock, Paper, or Scissors bundles will include this component
#[derive(Component)]
pub struct IsInFoodChain;

pub fn spawn_camera_ssys(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single()
        .expect("There is no window, how is this possible?");

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 7.0),
            ..default()
        }
    );
}

pub fn spawn_entities_ssys(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single()
        .expect("There is no window, how is this possible?");

    let texture = asset_server.load(format!("sprites/{}.png", Rock::STRING));
    for _ in  0..ENTITY_COUNT {
        spawn_entity::<Rock>(&mut commands, window, texture.clone());
    }

    let texture = asset_server.load(format!("sprites/{}.png", Paper::STRING));
    for _ in  0..ENTITY_COUNT {
        spawn_entity::<Paper>(&mut commands, window, texture.clone());
    }

    let texture = asset_server.load(format!("sprites/{}.png", Scissors::STRING));
    for _ in  0..ENTITY_COUNT {
        spawn_entity::<Scissors>(&mut commands, window, texture.clone());
    }
}

pub fn spawn_entity<T: Component + Default>(
    commands: &mut Commands,
    window: &Window, 
    texture: Handle<Image>
) {
    let transform = generate_random_transform(window);
    commands.spawn((
        SpriteBundle { transform, texture, ..default() },
        T::default(),
        IsInFoodChain,
    ));
}

pub fn generate_random_transform(window: &Window) -> Transform {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    Transform::from_xyz(random_x, random_y, 0.0)
}

// this is cool but might just make more sense to do in a single loop
pub fn movement_sys<O: Component, H: Component, L: Component>(
    mut own_query: Query<&mut Transform, With<O>>,
    predators_query: Query<&Transform, (With<H>, Without<O>)>,
    prey_query: Query<&Transform, (With<L>, Without<O>)>,
    time: Res<Time>
) {
    let predator_positions: Vec<Vec3> = predators_query.iter()
        .map(|t| { t.translation })
        .collect();

    let prey_positions: Vec<Vec3> = prey_query.iter()
        .map(|t| {t.translation })
        .collect();

    for mut transform in own_query.iter_mut() {
        let direction = get_own_direction(
            &transform, 
            predator_positions.clone(), 
            prey_positions.clone(),
        );

        if let Some(mut direction) = direction {
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            transform.translation += direction * ENTITY_SPEED * time.delta_seconds();
        }
    }
}

pub fn get_own_direction(
    transform: &Mut<Transform>, 
    predator_positions: Vec<Vec3>, 
    prey_positions: Vec<Vec3>
) -> Option<Vec3> {
    let translation = transform.translation;

    let closest_predator_position: Option<Vec3> = get_closest(translation, &predator_positions);
    let closest_prey_position: Option<Vec3> = get_closest(translation, &prey_positions);

    let run_away = if closest_predator_position.is_some() && closest_prey_position.is_some() {
        closest_predator_position.unwrap().distance(translation) < closest_prey_position.unwrap().distance(translation)
    } else if closest_predator_position.is_some() {
        true
    } else if closest_prey_position.is_some() {
        false
    } else {
        return None;
    };

    let direction = if run_away {
        let closest_predator_position = closest_predator_position.unwrap();
        Vec3::new(
            if translation.x - closest_predator_position.x >= 0.0 { 1.0 } else {-1.0},
            if translation.y - closest_predator_position.y >= 0.0 { 1.0 } else {-1.0},
            0.0
        )
    } else {
        let closest_prey_position = closest_prey_position.unwrap();
        Vec3::new(
            if translation.x - closest_prey_position.x >= 0.0 { -1.0 } else {1.0},
            if translation.y - closest_prey_position.y >= 0.0 { -1.0 } else {1.0},
            0.0
        )
    };

    Some(direction)
}

pub fn get_closest(target: Vec3, positions: &Vec<Vec3>) -> Option<Vec3> {
    positions.iter()
        .fold(None, |acc, t| { 
            if let Some(existing) = acc {
                if existing.distance(target) > t.distance(target) { 
                    Some(*t)
                } else { 
                    acc
                }
            } else {
                Some(*t)
            }
        })
}

pub fn contain_entities_sys(
    mut entities_query: Query<&mut Transform, With<IsInFoodChain>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = 0.0 - ENTITY_HALF_SIZE;
    let x_max = window.width() + ENTITY_HALF_SIZE;
    let y_min = 0.0 - ENTITY_HALF_SIZE;
    let y_max = window.height() + ENTITY_HALF_SIZE;
    for mut e in entities_query.iter_mut() {
        if e.translation.x < x_min {
            e.translation.x = x_max;
        } else if e.translation.x > x_max {
            e.translation.x = x_min;
        }

        if e.translation.y < y_min {
            e.translation.y = y_max;
        } else if e.translation.y > y_max {
            e.translation.y = y_min;
        }
    }
}

pub fn detect_collisions_from_predators_sys<O: Component, H: Component + Default + AssociatedString>(
    mut commands: Commands,
    mut own_query: Query<(&Transform, Entity), With<O>>,
    predators_query: Query<&Transform, (With<H>, Without<O>)>,
    asset_server: Res<AssetServer>
) {
    let predator_positions: Vec<Vec3> = predators_query.iter()
        .map(|t| { t.translation })
        .collect();

    for (transform, entity) in own_query.iter_mut() {
        let translation = transform.translation;

        let closest_predator_position: Option<Vec3> = get_closest(translation, &predator_positions);

        if let Some(closest_predator_position) = closest_predator_position {
            if closest_predator_position.distance(translation) < ENTITY_SIZE {
                commands.entity(entity)
                    .remove::<O>()
                    .remove::<SpriteBundle>()
                    .insert((
                        H::default(),
                        SpriteBundle {
                            transform: *transform,
                            texture: asset_server.load(format!("sprites/{}.png", H::STRING)),
                            ..default()
                        }
                    ));
            }
        }
    }
}

// pub fn maintain_personal_space_sys<T: Component>(
//     mut entity_query: Query<&mut Transform, With<T>>
// ) { 
//     let mut vv: Vec<Vec3> = vec!();
//     for mut current in entity_query.iter_mut() {
//         for existing_translation in vv {
//             if existing_translation.distance(current.translation) < (ENTITY_SIZE - 5.0) {
//                 current.translation += 
//             }
//         }
//         vv.push(current.translation.clone());
//     }
// }