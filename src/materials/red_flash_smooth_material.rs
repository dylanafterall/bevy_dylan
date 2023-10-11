use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "1f98b736-04b3-4f89-8057-336e811bf1cf"]
pub struct RedFlashSmoothMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for RedFlashSmoothMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/red_flash_smooth.wgsl".into()
    }
}
