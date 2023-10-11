use crate::materials::{
    blue_flash_material::BlueFlashMaterial, red_flash_material::RedFlashMaterial,
    red_flash_smooth_material::RedFlashSmoothMaterial,
    red_flash_tangent_material::RedFlashTangentMaterial, red_material::RedMaterial,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// -----------------------------------------------------------------------------
pub fn spawn_shader_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut red_materials: ResMut<Assets<RedMaterial>>,
    mut red_flash_materials: ResMut<Assets<RedFlashMaterial>>,
    mut red_flash_tangent_materials: ResMut<Assets<RedFlashTangentMaterial>>,
    mut red_flash_smooth_materials: ResMut<Assets<RedFlashSmoothMaterial>>,
    mut blue_flash_materials: ResMut<Assets<BlueFlashMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/white_square.png");

    // no shader ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: colors.add(ColorMaterial::from(texture_handle.clone())),
        transform: Transform::from_xyz(-100.0, 30.0, 0.0),
        ..default()
    });

    // red ---------------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: red_materials.add(RedMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-75.0, 30.0, 0.0),
        ..default()
    });

    // red flash ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: red_flash_materials.add(RedFlashMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-50.0, 30.0, 0.0),
        ..default()
    });

    // red flash smooth --------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: red_flash_smooth_materials.add(RedFlashSmoothMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-25.0, 30.0, 0.0),
        ..default()
    });

    // red flash tangent -------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: red_flash_tangent_materials.add(RedFlashTangentMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(0.0, 30.0, 0.0),
        ..default()
    });

    // blue flash --------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: blue_flash_materials.add(BlueFlashMaterial {
            color_texture: Some(texture_handle),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(25.0, 30.0, 0.0),
        ..default()
    });
}
