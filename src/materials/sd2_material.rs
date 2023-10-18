use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "8bba6ccf-a0f5-4d40-9288-e52406b27137"]
pub struct SD2Material {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for SD2Material {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/sd_test_2.wgsl".into()
    }
}
