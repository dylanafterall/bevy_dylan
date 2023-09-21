mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct FixedJointPlugin;

impl Plugin for FixedJointPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::First), (systems::spawn_fixed_joint,))
            .add_systems(
                Update,
                (systems::handle_destructible_contact.run_if(in_state(SceneState::First)),),
            );
    }
}
