use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "7a2c80fa-7da8-4a7c-b0f1-bb07b2cfb176"]
pub struct ShapeMorphMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for ShapeMorphMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/shape_morph.wgsl".into()
    }
}
