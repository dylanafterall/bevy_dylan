use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f603e034-9177-47d5-a927-6327d45e0211"]
pub struct MarbleEdgeMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for MarbleEdgeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/marble_edge.wgsl".into()
    }
}
