use bevy::{prelude::*, window::PrimaryWindow};

use crate::utils::*;
use crate::components::{AssociatedString, IsInFoodChain};

pub fn entity_movement<O: Component, H: Component, L: Component>(
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
            transform.translation += direction * ENTITY_SPEED * (time.delta_seconds() * TIME_FACTOR);
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

pub fn contain_entities(
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

pub fn detect_collisions_from_predators<O: Component, H: Component + Default + AssociatedString>(
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

pub fn maintain_personal_space<T: Component>(
    mut entity_query: Query<&mut Transform, With<T>>,
    time: Res<Time>,
) { 
    let mut vv: Vec<Vec3> = vec!();
    for mut current in entity_query.iter_mut() {
        for existing_translation in &vv {
            if existing_translation.distance(current.translation) < (ENTITY_SIZE + 5.0) {
                let direction = Vec3::new(
                    if current.translation.x - existing_translation.x >= 0.0 { 1.0 } else {-1.0},
                    if current.translation.y - existing_translation.y >= 0.0 { 1.0 } else {-1.0}, 
                    0.0
                );
                current.translation += direction * ENTITY_SPEED * (time.delta_seconds() * TIME_FACTOR);
            }
        }
        vv.push(current.translation.clone());
    }
}