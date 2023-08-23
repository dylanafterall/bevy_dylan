use crate::AppState;
use crate::game::SimulationState;

use bevy::{
    app::*,
    prelude::*,
};

// -----------------------------------------------------------------------------
pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && *app_state.get() != AppState::Game {
        next_app_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && *app_state.get() != AppState::MainMenu {
        next_app_state.set(AppState::MainMenu);
        next_simulation_state.set(SimulationState::Paused);
        println!("Entered AppState::MainMenu");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}