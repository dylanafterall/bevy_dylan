use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "b575a0cd-a432-4f00-ab0c-fc924395089c"]
pub struct SwirlyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for SwirlyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/swirly.wgsl".into()
    }
}
