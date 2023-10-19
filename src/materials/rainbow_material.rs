use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "1fef9c78-f057-48aa-b854-cfe1df84629a"]
pub struct RainbowMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for RainbowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/rainbow.wgsl".into()
    }
}
