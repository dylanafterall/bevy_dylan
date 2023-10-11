mod gizmos;
mod particles;
pub mod shaders;
mod text;
mod vector_graphics;

use gizmos::GizmosPlugin;
use particles::ParticlesPlugin;
use shaders::ShaderTestPlugin;
use text::TextRenderPlugin;
use vector_graphics::VectorGraphicsPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ShaderTestPlugin,
            ParticlesPlugin,
            GizmosPlugin,
            TextRenderPlugin,
            VectorGraphicsPlugin,
        ));
    }
}
