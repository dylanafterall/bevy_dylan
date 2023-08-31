mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RoundConvexPlugin;

impl Plugin for RoundConvexPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_systems(OnEnter(SceneState::First), (
                systems::spawn_round_convex,
            ));
    }
}