use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{Rock, Paper, Scissors, IsInFoodChain, AssociatedString, Velocity};
use crate::utils::{ENTITY_COUNT, generate_exclusive_transform};

pub fn spawn_camera(
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

pub fn spawn_entities(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single()
        .expect("There is no window, how is this possible?");

    let mut taken_positions: Vec<Vec3> = vec!();

    let texture = asset_server.load(format!("sprites/{}.png", Rock::STRING));
    for _ in  0..ENTITY_COUNT {
        spawn_entity::<Rock>(&mut commands, window, texture.clone(), &mut taken_positions);
    }

    let texture = asset_server.load(format!("sprites/{}.png", Paper::STRING));
    for _ in  0..ENTITY_COUNT {
        spawn_entity::<Paper>(&mut commands, window, texture.clone(), &mut taken_positions);
    }

    let texture = asset_server.load(format!("sprites/{}.png", Scissors::STRING));
    for _ in  0..ENTITY_COUNT {
        spawn_entity::<Scissors>(&mut commands, window, texture.clone(), &mut taken_positions);
    }
}

pub fn spawn_entity<T: Component + Default>(
    commands: &mut Commands,
    window: &Window, 
    texture: Handle<Image>,
    taken_positions: &mut Vec<Vec3>
) {
    let transform = generate_exclusive_transform(window, taken_positions);
    commands.spawn((
        SpriteBundle { transform, texture, ..default() },
        T::default(),
        IsInFoodChain,
        Velocity(Vec3::ZERO),
    ));
}
