pub mod events;
pub mod resources;
mod systems;

use resources::ResolutionSettings;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ConfigWindowPlugin;

impl Plugin for ConfigWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::ChangeResolution>()
            .insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::DARK_GRAY))
            .init_resource::<ResolutionSettings>()
            .add_systems(PreStartup, (systems::setup_window,))
            .add_systems(
                Update,
                (
                    systems::emit_resolution_change,
                    systems::handle_resolution_change,
                    systems::toggle_vsync,
                    systems::toggle_cursor,
                    systems::change_clear_color,
                ),
            );
    }
}
