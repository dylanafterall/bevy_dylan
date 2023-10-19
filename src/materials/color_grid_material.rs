use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "8cca320f-bced-4185-a4ff-873efd31c72e"]
pub struct ColorGridMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for ColorGridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/color_grid.wgsl".into()
    }
}
