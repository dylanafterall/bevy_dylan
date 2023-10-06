mod events;
mod systems;

mod camera_manager;
mod characters;
mod collision_manager;
mod environment;
pub mod objects;
pub mod render;
pub mod scene_manager;

use camera_manager::CameraManagerPlugin;
use characters::CharactersPlugin;
use collision_manager::CollisionManagerPlugin;
use environment::EnvironmentPlugin;
use objects::ObjectsPlugin;
use render::RenderPlugin;
use scene_manager::SceneManagerPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<events::TogglePause>()
            .add_plugins((
                CameraManagerPlugin,
                CollisionManagerPlugin,
                RenderPlugin,
                SceneManagerPlugin,
                CharactersPlugin,
                EnvironmentPlugin,
                ObjectsPlugin,
            ))
            .add_systems(
                Update,
                (
                    systems::emit_toggle_pause,
                    systems::handle_toggle_pause.run_if(not(in_state(GameState::Inert))),
                ),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Inert,
    Playing,
    Paused,
}
