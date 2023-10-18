use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "42c6c97b-13cf-46ed-8e3f-7725d3ef2127"]
pub struct SD4Material {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for SD4Material {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/sd_test_4.wgsl".into()
    }
}
