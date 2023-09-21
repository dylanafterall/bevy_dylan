mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_portal,));
    }
}
