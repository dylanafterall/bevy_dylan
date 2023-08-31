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
            ));
    }
}