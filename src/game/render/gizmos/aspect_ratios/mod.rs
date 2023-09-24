mod systems;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct AspectRatioPlugin;

impl Plugin for AspectRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (systems::spawn_aspect_ratio_gizmos,));
    }
}
