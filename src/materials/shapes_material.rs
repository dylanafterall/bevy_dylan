use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "eee67dc5-92eb-4c5a-b2ae-8b8d254f0408"]
pub struct ShapesMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for ShapesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/shapes.wgsl".into()
    }
}
