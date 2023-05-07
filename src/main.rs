mod utils;
mod components;
mod startup_systems;
mod systems;

use components::{Rock, Paper, Scissors};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_startup_system(startup_systems::spawn_camera)
        .add_startup_system(startup_systems::spawn_entities)

        .add_system(systems::entity_movement::<Rock, Paper, Scissors>)
        .add_system(systems::entity_movement::<Paper, Scissors, Rock>)
        .add_system(systems::entity_movement::<Scissors, Rock, Paper>)
        .add_system(systems::contain_entities)
        .add_system(systems::detect_collisions_from_predators::<Rock, Paper>)
        .add_system(systems::detect_collisions_from_predators::<Paper, Scissors>)
        .add_system(systems::detect_collisions_from_predators::<Scissors, Rock>)
        .add_system(systems::maintain_personal_space::<Rock>)
        .add_system(systems::maintain_personal_space::<Paper>)
        .add_system(systems::maintain_personal_space::<Scissors>)

        .run();
}


// this is cool but might just make more sense to do in a single loop