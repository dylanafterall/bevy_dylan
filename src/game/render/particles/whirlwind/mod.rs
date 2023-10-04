mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct WhirlwindPlugin;

impl Plugin for WhirlwindPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Third), (systems::spawn_whirlwind,));
    }
}
