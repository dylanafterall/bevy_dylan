use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "bf17aa3d-ed15-45a2-be2a-25e7b23b3b37"]
pub struct NoiseValueMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for NoiseValueMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/noise_value.wgsl".into()
    }
}
