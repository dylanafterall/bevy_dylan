mod components;
mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ElasticPlugin;

impl Plugin for ElasticPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_elastic,))
            .add_systems(
                Update,
                (systems::update_mesh_positions.run_if(in_state(SceneState::Fifth)),),
            );
    }
}
