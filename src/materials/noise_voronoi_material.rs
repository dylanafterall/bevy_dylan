use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// -----------------------------------------------------------------------------
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "c8478722-6ca2-487b-bc42-1d7675a00ea1"]
pub struct NoiseVoronoiMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material2d for NoiseVoronoiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/noise_voronoi.wgsl".into()
    }
}
