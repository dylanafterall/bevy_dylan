mod shader_test;

use self::shader_test::ShaderTestPlugin;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

// -----------------------------------------------------------------------------
pub struct MyMaterialsPlugin;

impl Plugin for MyMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ShaderTestPlugin,
        ));
    }
}

// NEAT ------------------------------------------------------------------------
// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct NeatMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for NeatMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/neat_material.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// -----------------------------------------------------------------------------
