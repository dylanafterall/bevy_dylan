use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "042d7ab4-e957-455c-90a3-78bbbd0d9f0c"]
pub struct ColorBlendMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for ColorBlendMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/color_blend.wgsl".into()
    }
}
