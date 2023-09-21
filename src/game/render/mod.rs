mod gizmos;
mod particles;

use gizmos::GizmosPlugin;
use particles::ParticlesPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ParticlesPlugin, GizmosPlugin));
    }
}
