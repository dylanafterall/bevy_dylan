pub mod blue_flash_material;
pub mod circle_material;
pub mod color_blend_material;
pub mod color_grid_material;
pub mod color_polar_material;
pub mod easing;
pub mod lava_lamp_material;
pub mod marble_edge_material;
pub mod noise_gradient_material;
pub mod noise_value_material;
pub mod noise_voronoi_material;
pub mod rainbow_material;
pub mod random_static;
pub mod red_flash_material;
pub mod red_flash_smooth_material;
pub mod red_flash_tangent_material;
pub mod red_material;
pub mod sd1_material;
pub mod sd2_material;
pub mod sd3_material;
pub mod sd4_material;
pub mod shape_collision_material;
pub mod shape_morph_material;
pub mod shapes_material;
pub mod swirly_material;
pub mod voronoi_smooth_material;
pub mod wipes_material;
pub mod wood_grain_material;

use self::{
    blue_flash_material::BlueFlashMaterial, circle_material::CircleMaterial,
    color_blend_material::ColorBlendMaterial, color_grid_material::ColorGridMaterial,
    color_polar_material::ColorPolarMaterial, easing::EasingMaterial,
    lava_lamp_material::LavaLampMaterial, marble_edge_material::MarbleEdgeMaterial,
    noise_gradient_material::NoiseGradientMaterial, noise_value_material::NoiseValueMaterial,
    noise_voronoi_material::NoiseVoronoiMaterial, rainbow_material::RainbowMaterial,
    random_static::RandomStaticMaterial, red_flash_material::RedFlashMaterial,
    red_flash_smooth_material::RedFlashSmoothMaterial,
    red_flash_tangent_material::RedFlashTangentMaterial, red_material::RedMaterial,
    sd1_material::SD1Material, sd2_material::SD2Material, sd3_material::SD3Material,
    sd4_material::SD4Material, shape_collision_material::ShapeCollisionMaterial,
    shape_morph_material::ShapeMorphMaterial, shapes_material::ShapesMaterial,
    swirly_material::SwirlyMaterial, voronoi_smooth_material::VoronoiSmoothMaterial,
    wipes_material::WipesMaterial, wood_grain_material::WoodGrainMaterial,
};
use bevy::{app::PluginGroupBuilder, prelude::PluginGroup, sprite::Material2dPlugin};

// -----------------------------------------------------------------------------
pub struct MaterialPluginGroup;

impl PluginGroup for MaterialPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(Material2dPlugin::<BlueFlashMaterial>::default())
            .add(Material2dPlugin::<CircleMaterial>::default())
            .add(Material2dPlugin::<ColorBlendMaterial>::default())
            .add(Material2dPlugin::<ColorGridMaterial>::default())
            .add(Material2dPlugin::<ColorPolarMaterial>::default())
            .add(Material2dPlugin::<EasingMaterial>::default())
            .add(Material2dPlugin::<LavaLampMaterial>::default())
            .add(Material2dPlugin::<MarbleEdgeMaterial>::default())
            .add(Material2dPlugin::<NoiseGradientMaterial>::default())
            .add(Material2dPlugin::<NoiseValueMaterial>::default())
            .add(Material2dPlugin::<NoiseVoronoiMaterial>::default())
            .add(Material2dPlugin::<RainbowMaterial>::default())
            .add(Material2dPlugin::<RandomStaticMaterial>::default())
            .add(Material2dPlugin::<RedFlashMaterial>::default())
            .add(Material2dPlugin::<RedFlashSmoothMaterial>::default())
            .add(Material2dPlugin::<RedFlashTangentMaterial>::default())
            .add(Material2dPlugin::<RedMaterial>::default())
            .add(Material2dPlugin::<SD1Material>::default())
            .add(Material2dPlugin::<SD2Material>::default())
            .add(Material2dPlugin::<SD3Material>::default())
            .add(Material2dPlugin::<SD4Material>::default())
            .add(Material2dPlugin::<ShapeCollisionMaterial>::default())
            .add(Material2dPlugin::<ShapeMorphMaterial>::default())
            .add(Material2dPlugin::<ShapesMaterial>::default())
            .add(Material2dPlugin::<SwirlyMaterial>::default())
            .add(Material2dPlugin::<VoronoiSmoothMaterial>::default())
            .add(Material2dPlugin::<WipesMaterial>::default())
            .add(Material2dPlugin::<WoodGrainMaterial>::default())
    }
}
