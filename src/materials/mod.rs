pub mod blue_flash_material;
pub mod easing;
pub mod random_static;
pub mod red_flash_material;
pub mod red_flash_smooth_material;
pub mod red_flash_tangent_material;
pub mod red_material;
pub mod sd1_material;
pub mod sd2_material;
pub mod sd3_material;
pub mod sd4_material;
pub mod shapes_material;

use self::{
    blue_flash_material::BlueFlashMaterial, easing::EasingMaterial,
    random_static::RandomStaticMaterial, red_flash_material::RedFlashMaterial,
    red_flash_smooth_material::RedFlashSmoothMaterial,
    red_flash_tangent_material::RedFlashTangentMaterial, red_material::RedMaterial,
    sd1_material::SD1Material, sd2_material::SD2Material, sd3_material::SD3Material,
    sd4_material::SD4Material, shapes_material::ShapesMaterial,
};
use bevy::{app::PluginGroupBuilder, prelude::PluginGroup, sprite::Material2dPlugin};

// -----------------------------------------------------------------------------
pub struct MaterialPluginGroup;

impl PluginGroup for MaterialPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(Material2dPlugin::<BlueFlashMaterial>::default())
            .add(Material2dPlugin::<EasingMaterial>::default())
            .add(Material2dPlugin::<RandomStaticMaterial>::default())
            .add(Material2dPlugin::<RedFlashMaterial>::default())
            .add(Material2dPlugin::<RedFlashSmoothMaterial>::default())
            .add(Material2dPlugin::<RedFlashTangentMaterial>::default())
            .add(Material2dPlugin::<RedMaterial>::default())
            .add(Material2dPlugin::<SD1Material>::default())
            .add(Material2dPlugin::<SD2Material>::default())
            .add(Material2dPlugin::<SD3Material>::default())
            .add(Material2dPlugin::<SD4Material>::default())
            .add(Material2dPlugin::<ShapesMaterial>::default())
    }
}
