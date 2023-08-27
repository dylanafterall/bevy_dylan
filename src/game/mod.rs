mod events;
mod systems;

mod characters;
mod environment;
mod objects;

use crate::game::characters::CharactersPlugin;
use crate::game::environment::EnvironmentPlugin;
use crate::game::objects::ObjectsPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()

            .add_event::<events::TogglePause>()

            .add_plugins((
                CharactersPlugin,
                EnvironmentPlugin,
                ObjectsPlugin,
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