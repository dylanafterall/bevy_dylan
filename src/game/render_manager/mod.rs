mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RenderManagerPlugin;

impl Plugin for RenderManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SceneState::Third), (
                systems::draw_regular_shapes,
                systems::draw_circle,
                systems::draw_ellipse,
                systems::draw_line,
                systems::draw_rectangle,
                systems::draw_polygon,
                systems::draw_rounded_polygon,
                systems::draw_path,
            ));
    }
}