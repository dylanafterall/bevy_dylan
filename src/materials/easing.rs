use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "1b447264-f3f0-4a22-b8e5-df66d6d41940"]
pub struct EasingMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for EasingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/easing_test.wgsl".into()
    }
}
