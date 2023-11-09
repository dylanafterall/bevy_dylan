use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct RedFlashTangentMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for RedFlashTangentMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/red_flash_tangent.wgsl".into()
    }
}
