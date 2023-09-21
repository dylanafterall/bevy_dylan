mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct PrismaticJointPlugin;

impl Plugin for PrismaticJointPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            OnEnter(SceneState::Second),
            (systems::spawn_prismatic_joint,),
        );
    }
}
