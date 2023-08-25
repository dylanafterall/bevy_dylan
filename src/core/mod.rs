mod resources;

mod audio;
mod camera;
mod diagnostic;
mod input;
mod window;

use crate::core::audio::CoreAudioPlugin;
use crate::core::camera::CoreCameraPlugin;
// use crate::core::diagnostic::CoreDiagnosticPlugin;
use crate::core::input::CoreInputPlugin;
use crate::core::window::CoreWindowPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                CoreAudioPlugin,
                CoreCameraPlugin,
                // CoreDiagnosticPlugin,
                CoreInputPlugin,
                CoreWindowPlugin,
            ));
    }
}
