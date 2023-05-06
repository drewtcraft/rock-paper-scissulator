use bevy::{prelude::*, window::PrimaryWindow, render::view::window};
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera_ssys)
        .add_startup_system(spawn_rocks_papers_scissors_ssys)
        // .add_system(rock_movement_sys)
        // .add_system(paper_movement_sys)
        // .add_system(scissors_movement_sys)
        .add_system(movement_sys::<Rock, Paper, Scissors>)
        .add_system(contain_entities_sys)
        .run();
}

pub const ENTITY_COUNT: u16 = 5;
pub const ENTITY_SPEED: f32 = 100.0;
pub const ENTITY_SIZE: f32 = 64.0;
pub const ENTITY_HALF_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Rock;

#[derive(Component)]
pub struct Paper;

#[derive(Component)]
pub struct Scissors;

pub enum RoShamBo {
    Rock,
    Paper,
    Scissors,
}

#[derive(Component)]
pub struct IsInFoodChain;

impl ToString for RoShamBo {
    fn to_string(&self) -> String {
        match self {
            RoShamBo::Rock => "rock".to_string(),
            RoShamBo::Paper => "paper".to_string(),
            RoShamBo::Scissors => "scissors".to_string(),
        }
    }
}

pub const ROSHAMBO_ENUM_VALUES: [RoShamBo; 3] = [RoShamBo::Rock, RoShamBo::Paper, RoShamBo::Scissors];

pub fn spawn_camera_ssys(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single()
        .expect("There is no window, how is this possible?");
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 7.0),
        ..default()
    });
}

pub fn spawn_rocks_papers_scissors_ssys(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {

    let window = window_query.get_single()
        .expect("There is no window, how is this possible?");

    for entity_type in ROSHAMBO_ENUM_VALUES {
        for _ in  0..ENTITY_COUNT {
            spawn_entity(&mut commands, window, &asset_server, &entity_type);
        }
    }
}

pub fn spawn_entity(
    commands: &mut Commands,
    window: &Window, 
    asset_server: &Res<AssetServer>,
    entity_type: &RoShamBo
) {
    let transform = generate_random_transform(window);
    let texture = asset_server.load(format!("sprites/{}.png", entity_type.to_string()));
    match entity_type {
        RoShamBo::Rock => {
            commands.spawn((
                SpriteBundle { transform, texture, ..default() },
                Rock,
                IsInFoodChain,
            ));
        },
        RoShamBo::Paper => {
            commands.spawn((
                SpriteBundle { transform, texture, ..default() },
                Paper,
                IsInFoodChain,
            ));
        }, 
        RoShamBo::Scissors => {
            commands.spawn((
                SpriteBundle { transform, texture, ..default() },
                Scissors,
                IsInFoodChain,
            ));
        }
    }
}

pub fn generate_random_transform(window: &Window) -> Transform {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    Transform::from_xyz(random_x, random_y, 0.0)
}

// can I throw <T, M, N, L> on here and specify the types that way??
pub fn movement_sys<O: Component, H: Component, L: Component>(
    mut own_query: Query<&mut Transform, With<O>>,
    predators_query: Query<&Transform, (With<H>, Without<O>)>,
    prey_query: Query<&Transform, (With<L>, Without<O>)>,
    time: Res<Time>
) {
    let predator_positions: Vec<&Vec3> = predators_query.iter()
        .map(|t| { &t.translation })
        .collect();

    let prey_positions: Vec<&Vec3> = prey_query.iter()
        .map(|t| {&t.translation })
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

pub fn rock_movement_sys(
    mut own_query: Query<&mut Transform, With<Rock>>,
    predators_query: Query<&Transform, (With<Paper>, Without<Rock>)>,
    prey_query: Query<&Transform, (With<Scissors>, Without<Rock>)>,
    time: Res<Time>
) {
    let predator_positions: Vec<&Vec3> = predators_query.iter()
        .map(|t| { &t.translation })
        .collect();

    let prey_positions: Vec<&Vec3> = prey_query.iter()
        .map(|t| {&t.translation })
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

pub fn paper_movement_sys(
    mut own_query: Query<&mut Transform, With<Paper>>,
    predators_query: Query<&Transform, (With<Rock>, Without<Paper>)>,
    prey_query: Query<&Transform, (With<Scissors>, Without<Paper>)>,
    time: Res<Time>
) {
    let predator_positions: Vec<&Vec3> = predators_query.iter()
        .map(|t| { &t.translation })
        .collect();

    let prey_positions: Vec<&Vec3> = prey_query.iter()
        .map(|t| {&t.translation })
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

pub fn scissors_movement_sys(
    mut own_query: Query<&mut Transform, With<Scissors>>,
    predators_query: Query<&Transform, (With<Rock>, Without<Scissors>)>,
    prey_query: Query<&Transform, (With<Paper>, Without<Scissors>)>,
    time: Res<Time>
) {
    let predator_positions: Vec<&Vec3> = predators_query.iter()
        .map(|t| { &t.translation })
        .collect();

    let prey_positions: Vec<&Vec3> = prey_query.iter()
        .map(|t| {&t.translation })
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
    predator_positions: Vec<&Vec3>, 
    prey_positions: Vec<&Vec3>
) -> Option<Vec3> {
    let translation = transform.translation;
    let closest_predator_position: Option<&Vec3> = predator_positions.iter()
        .fold(None, |acc, t| { 
            if let Some(existing) = acc {
                if existing.distance(translation) > t.distance(translation) { 
                    Some(t)
                } else { 
                    acc
                }
            } else {
                Some(t)
            }
        });

    let closest_prey_position: Option<&Vec3> = prey_positions.iter()
        .fold(None, |acc, t| {
            if let Some(existing) = acc {
                if existing.distance(translation) > t.distance(translation) { 
                    Some(t)
                } else { 
                    acc
                }
            } else {
                Some(t)
            }
        });

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