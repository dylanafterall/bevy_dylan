mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RectGizmoPlugin;

impl Plugin for RectGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            Update,
            systems::spawn_rects.run_if(in_state(SceneState::Third)),
        );
    }
}
