use bevy::transform::commands;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::{
    Angle, AssociatedString, IsInFoodChain, MainMenu, Paper, PauseButton, PlayButton, Rock,
    Scissors, Velocity,
};
use crate::{utils::*, AppState, PlayState};

pub fn entity_movement<O: Component, H: Component, L: Component>(
    mut own_query: Query<(&mut Transform, &mut Velocity, &mut Angle), With<O>>,
    predators_query: Query<&Transform, (With<H>, Without<O>)>,
    prey_query: Query<&Transform, (With<L>, Without<O>)>,
    time: Res<Time>,
) {
    let prey_positions: Vec<Vec3> = prey_query.iter().map(|t| t.translation).collect();

    let predator_positions: Vec<Vec3> = predators_query.iter().map(|t| t.translation).collect();

    for (mut transform, mut velocity, mut angle) in own_query.iter_mut() {
        let direction = get_own_direction(
            &transform,
            predator_positions.clone(),
            prey_positions.clone(),
        );

        if let Some(mut direction) = direction {
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            // set new angle unless pointing at proper direction already
            // if NOT pointed in the right direction, should incur a movement penalty
            // where 180 difference is 0 movement and facing the right angle is full movement

            let angle_as_radians = angle.0 / 180.0 * std::f32::consts::PI;
            let anglee = Vec3::new(angle.0.sin(), angle.0.cos(), 0.0);
            let diff = direction.angle_between(anglee);

            let accel_modifier = if diff < 0.5 {
                angle.0 -= diff;
                ENTITY_ACCELERATION
            } else if diff >= 0.5 && diff < 1.5 {
                angle.0 += 0.1;
                ENTITY_ACCELERATION * -2.0
            } else if diff >= 1.5 {
                angle.0 -= 0.1;
                ENTITY_ACCELERATION * -2.0
            } else {
                0.0
            };
            transform.rotation = Quat::from_rotation_z(angle.0);

            velocity.0 += (direction * accel_modifier).clamp(
                Vec3::new(-ENTITY_MAX_SPEED, -ENTITY_MAX_SPEED, 0.0),
                Vec3::new(ENTITY_MAX_SPEED, ENTITY_MAX_SPEED, 0.0),
            );
            transform.translation += velocity.0 * (time.delta_seconds() * TIME_FACTOR);
        }
    }
}

pub fn get_own_direction(
    transform: &Mut<Transform>,
    predator_positions: Vec<Vec3>,
    prey_positions: Vec<Vec3>,
) -> Option<Vec3> {
    let translation = transform.translation;

    let closest_predator_position: Option<Vec3> = get_closest(translation, &predator_positions);
    let closest_prey_position: Option<Vec3> = get_closest(translation, &prey_positions);

    let run_away = if closest_predator_position.is_some() && closest_prey_position.is_some() {
        closest_predator_position.unwrap().distance(translation)
            < closest_prey_position.unwrap().distance(translation)
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
            if translation.x - closest_predator_position.x >= 0.0 {
                1.0
            } else {
                -1.0
            },
            if translation.y - closest_predator_position.y >= 0.0 {
                1.0
            } else {
                -1.0
            },
            0.0,
        )
    } else {
        let closest_prey_position = closest_prey_position.unwrap();
        Vec3::new(
            if translation.x - closest_prey_position.x >= 0.0 {
                -1.0
            } else {
                1.0
            },
            if translation.y - closest_prey_position.y >= 0.0 {
                -1.0
            } else {
                1.0
            },
            0.0,
        )
    };

    Some(direction)
}

pub fn get_closest(target: Vec3, positions: &Vec<Vec3>) -> Option<Vec3> {
    positions.iter().fold(None, |acc, t| {
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
    asset_server: Res<AssetServer>,
) {
    let predator_positions: Vec<Vec3> = predators_query.iter().map(|t| t.translation).collect();

    for (transform, entity) in own_query.iter_mut() {
        let translation = transform.translation;

        let closest_predator_position: Option<Vec3> = get_closest(translation, &predator_positions);

        if let Some(closest_predator_position) = closest_predator_position {
            if closest_predator_position.distance(translation) < ENTITY_SIZE {
                commands
                    .entity(entity)
                    .remove::<O>()
                    .remove::<SpriteBundle>()
                    .insert((
                        H::default(),
                        SpriteBundle {
                            transform: *transform,
                            texture: asset_server.load(format!("sprites/{}.png", H::STRING)),
                            ..default()
                        },
                    ));
            }
        }
    }
}

// TODO: this function name is fun but this whole thing
//  could be folded into the movement system
pub fn maintain_personal_space<T: Component>(
    mut entity_query: Query<(&mut Transform, &mut Velocity), With<T>>,
    time: Res<Time>,
) {
    let mut vv: Vec<Vec3> = vec![];
    for (mut current, mut velocity) in entity_query.iter_mut() {
        for existing_translation in &vv {
            if existing_translation.distance(current.translation) < (ENTITY_SIZE + 5.0) {
                let direction = Vec3::new(
                    if current.translation.x - existing_translation.x >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    },
                    if current.translation.y - existing_translation.y >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    },
                    0.0,
                );

                // accelerate faster when avoiding same type of self
                velocity.0 += (direction * ENTITY_ACCELERATION * 3.0).clamp(
                    Vec3::new(-ENTITY_MAX_SPEED, -ENTITY_MAX_SPEED, 0.0),
                    Vec3::new(ENTITY_MAX_SPEED, ENTITY_MAX_SPEED, 0.0),
                );
                current.translation += velocity.0 * (time.delta_seconds() * TIME_FACTOR);
            }
        }
        vv.push(current.translation.clone());
    }
}

pub fn is_game_over(
    rocks_query: Query<&Rock>,
    papers_query: Query<&Paper>,
    scissors_query: Query<&Scissors>,
    mut next_game_state: ResMut<NextState<AppState>>,
) {
    let no_rocks = rocks_query.is_empty();
    let no_papers = papers_query.is_empty();
    let no_scissors = scissors_query.is_empty();
    let my_stuff: [bool; 3] = [no_rocks, no_papers, no_scissors];
    if my_stuff.iter().any(|f| *f) {
        println!("game over!!!!");
        next_game_state.set(AppState::MainMenu);
    }
}

pub const MAIN_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Percent(8.0), Val::Percent(8.0)),
    ..Style::DEFAULT
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // title
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Rock Paper Scissulator",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 64.0,
                            color: Color::BLACK,
                        },
                    )],
                    ..default()
                },
                ..default()
            });
            // play button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                            ..Style::DEFAULT
                        },
                        background_color: BackgroundColor(Color::RED),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                    font_size: 64.0,
                                    color: Color::BLACK,
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu) = main_menu_query.get_single() {
        commands.entity(main_menu).despawn_recursive();
    }
}

pub fn spawn_play_toggle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                    ..Style::default()
                },
                ..default()
            },
            PauseButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Play/Pause",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 64.0,
                            color: Color::BLACK,
                        },
                    )],
                    ..default()
                },
                ..default()
            });
        });
}

pub fn despawn_play_toggle(
    mut commands: Commands,
    toggle_button_query: Query<Entity, With<PauseButton>>,
) {
    if let Ok(toggle_button) = toggle_button_query.get_single() {
        commands.entity(toggle_button).despawn_recursive();
    }
}

pub fn play_toggle_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<PauseButton>, Changed<Interaction>),
    >,
    mut next_simulation_state: ResMut<NextState<PlayState>>,
    current_simulation_state: Res<State<PlayState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                let next_play_state = if current_simulation_state.0 == PlayState::Playing {
                    PlayState::Paused
                } else {
                    PlayState::Playing
                };
                next_simulation_state.set(next_play_state);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::BLUE);
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::RED);
            }
        }
    }
}

pub fn play_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<PlayButton>, Changed<Interaction>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                next_app_state.set(AppState::SimulationRunning);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::BLUE);
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::RED);
            }
        }
    }
}
