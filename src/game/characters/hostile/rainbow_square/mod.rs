mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RainbowSquarePlugin;

impl Plugin for RainbowSquarePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::First), (systems::spawn_rainbow_square,));
    }
}
