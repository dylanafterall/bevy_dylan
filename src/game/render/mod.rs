mod gizmos;
pub mod materials;
mod particles;
mod text;
mod vector_graphics;

use gizmos::GizmosPlugin;
use materials::MyMaterialsPlugin;
use particles::ParticlesPlugin;
use text::TextRenderPlugin;
use vector_graphics::VectorGraphicsPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MyMaterialsPlugin,
            ParticlesPlugin,
            GizmosPlugin,
            TextRenderPlugin,
            VectorGraphicsPlugin,
        ));
    }
}
