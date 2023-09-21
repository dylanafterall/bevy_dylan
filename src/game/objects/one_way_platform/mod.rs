mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct OneWayPlatformPlugin;

impl Plugin for OneWayPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(
            OnEnter(SceneState::Fourth),
            (systems::spawn_one_way_platform,),
        );
    }
}
