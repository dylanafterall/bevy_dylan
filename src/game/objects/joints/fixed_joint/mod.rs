mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct FixedJointPlugin;

impl Plugin for FixedJointPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_systems(OnEnter(SceneState::Second), (
                systems::spawn_fixed_joint,
            ));
    }
}