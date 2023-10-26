use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "6c27ca07-c84a-4739-821c-f6ed584c86a1"]
pub struct LavaLampMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for LavaLampMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/lava_lamp.wgsl".into()
    }
}
