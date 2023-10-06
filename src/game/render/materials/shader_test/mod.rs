mod systems;

use crate::game::scene_manager::SceneState;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ShaderTestPlugin;

impl Plugin for ShaderTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Sixth), systems::spawn_shader_test);
    }
}
