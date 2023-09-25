mod components;
mod events;
mod systems;

use crate::game::GameState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::CameraMove>()
            .add_event::<events::CameraZoomIn>()
            .add_event::<events::CameraZoomOut>()
            .add_systems(Startup, (systems::spawn_cameras,))
            .add_systems(
                Update,
                (
                    systems::handle_resolution_change,
                    systems::emit_camera_move.run_if(in_state(GameState::Playing)),
                    systems::handle_camera_move.run_if(in_state(GameState::Playing)),
                    systems::emit_camera_zoom_in.run_if(in_state(GameState::Playing)),
                    systems::emit_camera_zoom_out.run_if(in_state(GameState::Playing)),
                    systems::handle_camera_zoom_in.run_if(in_state(GameState::Playing)),
                    systems::handle_camera_zoom_out.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}
