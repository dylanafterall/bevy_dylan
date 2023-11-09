use crate::events::*;

use crate::game::scene_manager::SceneState;
use crate::game::GameState;
use crate::AppState;

use bevy::{app::*, prelude::*};

// -----------------------------------------------------------------------------
pub fn emit_transition_to_title(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<TransitionAppState>,
) {
    if keyboard_input.just_pressed(KeyCode::T) {
        event_source.send(TransitionAppState {
            desired_app_state: AppState::Title,
        });
        keyboard_input.reset(KeyCode::T);
    }
}

pub fn emit_transition_to_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<TransitionAppState>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        event_source.send(TransitionAppState {
            desired_app_state: AppState::Game,
        });
        keyboard_input.reset(KeyCode::G);
    }
}

// -----------------------------------------------------------------------------
// handle transition -----------------------------------------------------------
pub fn handle_transition_app_state(
    current_app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_scene_state: ResMut<NextState<SceneState>>,
    mut event_listener: EventReader<TransitionAppState>,
) {
    for transition in event_listener.read() {
        let _desired_state = match transition.desired_app_state {
            AppState::Title => {
                let _current_state = match *current_app_state.get() {
                    AppState::Splash => {
                        // splash -> title
                        next_app_state.set(AppState::Title);
                    }
                    AppState::Settings => {
                        // settings -> title
                        next_app_state.set(AppState::Title);
                    }
                    AppState::Game => {
                        // game -> title
                        next_app_state.set(AppState::Title);
                        next_game_state.set(GameState::Inert);
                        next_scene_state.set(SceneState::Inert);
                    }
                    AppState::Fail => {
                        // fail -> title
                        next_app_state.set(AppState::Title);
                    }
                    _ => {}
                };
            }
            AppState::Settings => {
                let _current_state = match *current_app_state.get() {
                    AppState::Title => {
                        // title -> settings
                        next_app_state.set(AppState::Settings);
                    }
                    _ => {}
                };
            }
            AppState::Game => {
                let _current_state = match *current_app_state.get() {
                    AppState::Title => {
                        // title -> game
                        next_app_state.set(AppState::Game);
                        next_game_state.set(GameState::Playing);
                        next_scene_state.set(SceneState::First);
                    }
                    AppState::Fail => {
                        // fail -> game
                        next_app_state.set(AppState::Game);
                        next_game_state.set(GameState::Playing);
                        next_scene_state.set(SceneState::First);
                    }
                    _ => {}
                };
            }
            AppState::Fail => {
                let _current_state = match *current_app_state.get() {
                    AppState::Game => {
                        // game -> fail
                        next_app_state.set(AppState::Fail);
                        next_game_state.set(GameState::Inert);
                        next_scene_state.set(SceneState::Inert);
                    }
                    _ => {}
                };
            }
            _ => {} // invalid destinations (e.g. cannot transition -> splash)
        };
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
