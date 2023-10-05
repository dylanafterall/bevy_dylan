mod gizmos;
mod particles;
mod text;
mod vector_graphics;

use gizmos::GizmosPlugin;
use particles::ParticlesPlugin;
use text::TextRenderPlugin;
use vector_graphics::VectorGraphicsPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ParticlesPlugin,
            GizmosPlugin,
            TextRenderPlugin,
            VectorGraphicsPlugin,
        ));
    }
}
