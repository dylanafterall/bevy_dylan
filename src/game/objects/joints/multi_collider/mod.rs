mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct MultiColliderPlugin;

impl Plugin for MultiColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            OnEnter(SceneState::Second),
            (systems::spawn_multi_collider,),
        );
    }
}
