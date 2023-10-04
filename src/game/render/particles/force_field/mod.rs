mod components;
mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ForceFieldPlugin;

impl Plugin for ForceFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_force_field,))
            .add_systems(Update, systems::update.run_if(in_state(SceneState::Fifth)));
    }
}
