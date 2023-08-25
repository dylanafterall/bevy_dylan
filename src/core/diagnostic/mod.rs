use bevy::{
    prelude::*,
    diagnostic::{
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
};

// -----------------------------------------------------------------------------
pub struct CoreDiagnosticPlugin;

impl Plugin for CoreDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin,
            ));
    }
}