mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct LineGizmoPlugin;

impl Plugin for LineGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            Update,
            systems::spawn_lines.run_if(in_state(SceneState::Third)),
        );
    }
}
