mod components;
mod events;
mod systems;

use crate::AppState;
use crate::game::GameState;

use bevy::{
    prelude::*,
};

// -----------------------------------------------------------------------------
pub struct CoreCameraPlugin;

impl Plugin for CoreCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::CameraZoomIn>()
            .add_event::<events::CameraZoomOut>()

            .add_systems(Startup, (
                systems::spawn_world_and_ui_cameras,
            ))
            .add_systems(OnEnter(AppState::Game), (
                systems::spawn_player_camera,
            ))
            .add_systems(OnExit(AppState::Game), (
                systems::despawn_player_camera,
            ))
            .add_systems(Update, (
                systems::handle_camera_zoom_in.run_if(in_state(GameState::Playing)),
                systems::handle_camera_zoom_out.run_if(in_state(GameState::Playing)),
            ));
    }
}