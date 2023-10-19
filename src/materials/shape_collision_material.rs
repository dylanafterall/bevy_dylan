use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "5bac24f6-ce30-495f-84c6-b844c8c785ee"]
pub struct ShapeCollisionMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for ShapeCollisionMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/shape_collision.wgsl".into()
    }
}
