use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "2cb5b3e7-c4ed-46df-9377-6d41100af0ab"]
pub struct VoronoiSmoothMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for VoronoiSmoothMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/voronoi_smooth.wgsl".into()
    }
}
