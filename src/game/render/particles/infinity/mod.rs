mod components;
mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct InfinityPlugin;

impl Plugin for InfinityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Third), (systems::spawn_infinity,))
            .add_systems(Update, systems::update.run_if(in_state(SceneState::Third)));
    }
}
