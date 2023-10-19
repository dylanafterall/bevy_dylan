use bevy::{
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "042d7ab4-e957-455c-90a3-78bbbd0d9f0c"]
pub struct ColorBlendMaterial {}

impl Material2d for ColorBlendMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/color_blend.wgsl".into()
    }
}
