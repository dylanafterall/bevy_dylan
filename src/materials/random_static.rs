use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a1b0f55d-6c49-40ff-99a2-24f654ebc455"]
pub struct RandomStaticMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for RandomStaticMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/random_static.wgsl".into()
    }
}
