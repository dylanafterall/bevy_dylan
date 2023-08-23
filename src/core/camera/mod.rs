mod systems;

use bevy::{
    prelude::*,
};

// -----------------------------------------------------------------------------
pub struct CoreCameraPlugin;

impl Plugin for CoreCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                systems::spawn_camera,
            ));
    }
}