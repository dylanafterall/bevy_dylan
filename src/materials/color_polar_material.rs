use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "2dcae97e-c040-4615-9f40-76383d2094dd"]
pub struct ColorPolarMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for ColorPolarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/color_polar.wgsl".into()
    }
}
