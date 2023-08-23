mod systems;

mod character;
mod environment;
mod player;

use crate::AppState;
use crate::game::character::CharacterPlugin;
use crate::game::environment::EnvironmentPlugin;
use crate::game::player::PlayerPlugin;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // states
            .add_state::<SimulationState>()
            // plugins
            .add_plugins((
                CharacterPlugin,
                EnvironmentPlugin,
                PlayerPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, (
                systems::setup_physics,
            ))
            .add_systems(OnEnter(AppState::Game), (
                systems::pause_simulation,
            ))
            .add_systems(Update, (
                systems::toggle_simulation,
            ))
            .add_systems(OnExit(AppState::Game), (
                systems::resume_simulation,
            ));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}