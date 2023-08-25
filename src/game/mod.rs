mod events;
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
            .add_state::<GameState>()

            .add_event::<events::TogglePause>()

            .add_plugins((
                CharacterPlugin,
                EnvironmentPlugin,
                PlayerPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default(),
            ))

            .add_systems(OnEnter(AppState::Game), (
                systems::setup_physics,
            ))
            .add_systems(Update, (
                systems::emit_toggle_pause,
                systems::handle_toggle_pause.run_if(not(in_state(GameState::Inert))),
            ));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Inert,
    Playing,
    Paused,
}