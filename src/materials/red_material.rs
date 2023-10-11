use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "c664b2fb-93be-46ae-b8a0-c8b4085de6d9"]
pub struct RedMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for RedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/red.wgsl".into()
    }
}
