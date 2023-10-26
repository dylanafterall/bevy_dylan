use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "1dd904d8-852a-4a63-94a3-2751ffe767c6"]
pub struct NoiseGradientMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for NoiseGradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/noise_gradient.wgsl".into()
    }
}
