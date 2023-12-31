use super::events::*;
use crate::game::GameState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn _pause_game(mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Paused);
}

pub fn _resume_game(mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Playing);
}

pub fn emit_toggle_pause(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<TogglePause>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event_source.send(TogglePause {});
        keyboard_input.reset(KeyCode::Space);
    }
}

pub fn handle_toggle_pause(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut event_listener: EventReader<TogglePause>,
) {
    for _ in event_listener.read() {
        // toggle_pause run criteria is that GameState not be "inert"
        if *game_state.get() == GameState::Playing {
            commands.insert_resource(NextState(Some(GameState::Paused)));
            println!("Game Paused");
        } else {
            commands.insert_resource(NextState(Some(GameState::Playing)));
            println!("Game Playing");
        }
    }
}
