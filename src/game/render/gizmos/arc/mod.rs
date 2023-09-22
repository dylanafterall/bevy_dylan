mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ArcGizmoPlugin;

impl Plugin for ArcGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            Update,
            systems::spawn_arcs.run_if(in_state(SceneState::Third)),
        );
    }
}
