mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RedParticlesPlugin;

impl Plugin for RedParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_red_particles,));
    }
}
