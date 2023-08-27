use super::events::*;

use super::LevelState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn handle_transition_to_first_level(
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut event_listener: EventReader<TransitionToFirstLevel>,
) {
    for _ in event_listener.iter() {
        next_level_state.set(LevelState::First);
        println!("Entered LevelState::First");
    }
}

pub fn handle_transition_to_second_level(
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut event_listener: EventReader<TransitionToSecondLevel>,
) {
    for _ in event_listener.iter() {
        next_level_state.set(LevelState::Second);
        println!("Entered LevelState::Second");
    }
}

pub fn handle_transition_to_third_level(
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut event_listener: EventReader<TransitionToThirdLevel>,
) {
    for _ in event_listener.iter() {
        next_level_state.set(LevelState::Third);
        println!("Entered LevelState::Third");
    }
}