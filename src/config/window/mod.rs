mod resources;
mod systems;

use resources::ResolutionSettings;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ConfigWindowPlugin;

impl Plugin for ConfigWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::DARK_GRAY))
            .init_resource::<ResolutionSettings>()
            .add_systems(PreStartup, (systems::setup_window,))
            .add_systems(
                Update,
                (
                    systems::toggle_resolution,
                    systems::toggle_vsync,
                    systems::toggle_cursor,
                    systems::change_clear_color,
                ),
            );
    }
}
