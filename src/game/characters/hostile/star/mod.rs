mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::First), (systems::spawn_star,));
    }
}
