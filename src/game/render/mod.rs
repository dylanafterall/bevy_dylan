mod gizmos;
mod particles;
mod vector_graphics;

use gizmos::GizmosPlugin;
use particles::ParticlesPlugin;
use vector_graphics::VectorGraphicsPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ParticlesPlugin, GizmosPlugin, VectorGraphicsPlugin));
    }
}
