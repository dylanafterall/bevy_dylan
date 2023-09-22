mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RayGizmoPlugin;

impl Plugin for RayGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            Update,
            systems::spawn_rays.run_if(in_state(SceneState::Third)),
        );
    }
}
