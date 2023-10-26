use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "e12e08da-fb2f-4ebd-9686-ae4a4f5973d3"]
pub struct WipesMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for WipesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/wipes.wgsl".into()
    }
}
