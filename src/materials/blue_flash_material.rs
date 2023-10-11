use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "1f1082f4-5253-4676-9792-cefb8a2555b5"]
pub struct BlueFlashMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for BlueFlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/blue_flash.wgsl".into()
    }
}
