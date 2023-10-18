use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "0e851359-f915-47b1-b567-016b3dbad34c"]
pub struct SD3Material {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for SD3Material {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/sd_test_3.wgsl".into()
    }
}
