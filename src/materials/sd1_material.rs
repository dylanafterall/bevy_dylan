use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "fd4a453a-5271-4590-89ad-42c96bdd530f"]
pub struct SD1Material {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for SD1Material {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/sd_test_1.wgsl".into()
    }
}
