use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
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
