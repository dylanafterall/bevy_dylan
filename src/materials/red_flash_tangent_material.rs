use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "ba03c774-e353-40ec-835f-507c9ac59a1a"]
pub struct RedFlashTangentMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for RedFlashTangentMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/red_flash_tangent.wgsl".into()
    }
}
