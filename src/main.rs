mod components;
mod startup_systems;
mod systems;
mod utils;

use components::{Paper, Rock, Scissors};

use bevy::prelude::*;
use systems::{despawn_main_menu, spawn_main_menu};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SimulationRunning,
    SimulationOver,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PlayState {
    #[default]
    Playing,
    Paused,
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (startup_systems::spawn_entities, systems::spawn_play_toggle)
                .in_schedule(OnEnter(AppState::SimulationRunning)),
        )
        .add_systems((systems::play_toggle_interaction,))
        .add_systems(
            (
                systems::entity_movement::<Rock, Paper, Scissors>,
                systems::entity_movement::<Paper, Scissors, Rock>,
                systems::entity_movement::<Scissors, Rock, Paper>,
                systems::contain_entities,
                systems::detect_collisions_from_predators::<Rock, Paper>,
                systems::detect_collisions_from_predators::<Paper, Scissors>,
                systems::detect_collisions_from_predators::<Scissors, Rock>,
                systems::maintain_personal_space::<Rock>,
                systems::maintain_personal_space::<Paper>,
                systems::maintain_personal_space::<Scissors>,
                systems::is_game_over,
            )
                .in_set(OnUpdate(AppState::SimulationRunning))
                .in_set(OnUpdate(PlayState::Playing)),
        )
        .add_systems(
            (
                systems::despawn_play_toggle,
                startup_systems::despawn_entities,
            )
                .in_schedule(OnExit(AppState::SimulationRunning)),
        );
    }
}

pub struct SimulationOverPage;

impl Plugin for SimulationOverPage {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                systems::spawn_simulation_over_page,
                systems::spawn_simulation_over_timer,
            )
                .in_schedule(OnEnter(AppState::SimulationOver)),
        )
        .add_systems(
            (systems::tick_simulation_over_timer,).in_set(OnUpdate(AppState::SimulationOver)),
        )
        .add_systems(
            (
                systems::despawn_simulation_over_page,
                systems::despawn_simulation_over_timer,
            )
                .in_schedule(OnExit(AppState::SimulationOver)),
        );
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(systems::play_button_interaction);
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_state::<PlayState>()
        .add_startup_system(startup_systems::spawn_camera)
        .add_plugin(SimulationPlugin)
        .add_plugin(SimulationOverPage)
        .add_plugin(MainMenuPlugin)
        .run();
}
