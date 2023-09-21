mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct GravShiftPlugin;

impl Plugin for GravShiftPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::First), (systems::spawn_grav_shift,))
            .add_systems(OnExit(SceneState::First), (systems::reset_grav_shift,))
            .add_systems(Update, (systems::handle_grav_shift,));
    }
}
