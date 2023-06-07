use bevy::prelude::*;
use rand::prelude::*;

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

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Angle(pub f32);

impl Angle {
    pub fn new() -> Self {
        Self(random::<f32>() * 360.0)
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct PauseButton;

#[derive(Component)]
pub struct SimulationPage;

#[derive(Resource)]
pub struct SimulationOverTimer {
    timer: Timer,
}
