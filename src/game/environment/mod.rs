mod events;
mod systems;

mod first_level;
mod second_level;
mod third_level;

use crate::game::environment::first_level::FirstLevelPlugin;
use crate::game::environment::second_level::SecondLevelPlugin;
use crate::game::environment::third_level::ThirdLevelPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FirstLevelPlugin,
                SecondLevelPlugin,
                ThirdLevelPlugin,
            ))

            .add_state::<LevelState>()

            .add_event::<events::TransitionToFirstLevel>()
            .add_event::<events::TransitionToSecondLevel>()
            .add_event::<events::TransitionToThirdLevel>()

            .add_systems(Update, (
                systems::handle_transition_to_first_level.run_if(not(in_state(LevelState::First))),
                systems::handle_transition_to_second_level.run_if(not(in_state(LevelState::Second))),
                systems::handle_transition_to_third_level.run_if(not(in_state(LevelState::Third))),
            ));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LevelState {
    #[default]
    First,
    Second,
    Third,
}