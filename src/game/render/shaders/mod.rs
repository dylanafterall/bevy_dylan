mod systems;

use crate::game::scene_manager::SceneState;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ShaderTestPlugin;

impl Plugin for ShaderTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Sixth),
            (
                systems::spawn_first_row_shaders,
                systems::spawn_second_row_shaders,
                systems::spawn_third_row_shaders,
                systems::spawn_fourth_row_shaders,
            ),
        );
    }
}
