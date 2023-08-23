use crate::AppState;
use crate::game::SimulationState;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>
) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>
) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if *app_state.get() == AppState::Game && keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused");
        }
        if *simulation_state.get() == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation Running");
        }
    }
}

// -----------------------------------------------------------------------------
// RAPIER_2D DEMO CODE
pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}