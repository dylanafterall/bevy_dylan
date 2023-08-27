use crate::events::*;

use crate::AppState;
use crate::game::GameState;

use bevy::{
    app::*,
    prelude::*,
};

// -----------------------------------------------------------------------------
pub fn emit_transition_to_title(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<TransitionToTitle>,
) {
    if keyboard_input.just_pressed(KeyCode::T) {
        event_source.send(TransitionToTitle {});
        keyboard_input.reset(KeyCode::T);
    }
}

pub fn emit_transition_to_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<TransitionToGame>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        event_source.send(TransitionToGame {});
        keyboard_input.reset(KeyCode::G);
    }
}

// -----------------------------------------------------------------------------
// to title --------------------------------------------------------------------
pub fn handle_transition_splash_to_title(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_listener: EventReader<TransitionToTitle>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Title);
        println!("Entered AppState::Title");
    }
}

pub fn handle_transition_settings_to_title(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_listener: EventReader<TransitionToTitle>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Title);
        println!("Entered AppState::Title");
    }
}

pub fn handle_transition_game_to_title(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut event_listener: EventReader<TransitionToTitle>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Title);
        next_game_state.set(GameState::Inert);
        println!("Entered AppState::Title");
    }
}

// to settings -----------------------------------------------------------------
pub fn handle_transition_title_to_settings(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_listener: EventReader<TransitionToSettings>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Settings);
        println!("Entered AppState::Settings");
    }
}

// to game ---------------------------------------------------------------------
pub fn handle_transition_title_to_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut event_listener: EventReader<TransitionToGame>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Game);
        next_game_state.set(GameState::Playing);
        println!("Entered AppState::Game");
    }
}

pub fn handle_transition_fail_to_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut event_listener: EventReader<TransitionToGame>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Game);
        next_game_state.set(GameState::Playing);
        println!("Entered AppState::Game");
    }
}

// to fail ---------------------------------------------------------------------
pub fn handle_transition_game_to_fail(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut event_listener: EventReader<TransitionToFail>,
) {
    for _ in event_listener.iter() {
        next_app_state.set(AppState::Fail);
        next_game_state.set(GameState::Inert);
        println!("Entered AppState::Fail");
    }
}

// -----------------------------------------------------------------------------
pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}