mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CircleGizmoPlugin;

impl Plugin for CircleGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            Update,
            systems::spawn_circles.run_if(in_state(SceneState::Third)),
        );
    }
}
