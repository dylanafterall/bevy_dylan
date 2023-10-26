use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a0f2f987-4786-482d-8cb4-ea0830d74594"]
pub struct WoodGrainMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for WoodGrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/wood_grain.wgsl".into()
    }
}
