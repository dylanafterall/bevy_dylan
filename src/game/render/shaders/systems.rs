use crate::materials::{
    blue_flash_material::BlueFlashMaterial, easing::EasingMaterial,
    random_static::RandomStaticMaterial, red_flash_material::RedFlashMaterial,
    red_flash_smooth_material::RedFlashSmoothMaterial,
    red_flash_tangent_material::RedFlashTangentMaterial, red_material::RedMaterial,
    sd1_material::SD1Material, sd2_material::SD2Material, sd3_material::SD3Material,
    sd4_material::SD4Material, shapes_material::ShapesMaterial,
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
    mut easing_materials: ResMut<Assets<EasingMaterial>>,
    mut random_static_materials: ResMut<Assets<RandomStaticMaterial>>,
    mut shapes_materials: ResMut<Assets<ShapesMaterial>>,
    mut sd1_materials: ResMut<Assets<SD1Material>>,
    mut sd2_materials: ResMut<Assets<SD2Material>>,
    mut sd3_materials: ResMut<Assets<SD3Material>>,
    mut sd4_materials: ResMut<Assets<SD4Material>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/white_square.png");

    // -------------------------------------------------------------------------
    // row 1 -------------------------------------------------------------------
    // -------------------------------------------------------------------------

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

    // easing ------------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: easing_materials.add(EasingMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(25.0, 30.0, 0.0),
        ..default()
    });

    // blue flash --------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: blue_flash_materials.add(BlueFlashMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(50.0, 30.0, 0.0),
        ..default()
    });

    // -------------------------------------------------------------------------
    // row 2 -------------------------------------------------------------------
    // -------------------------------------------------------------------------

    // shapes ------------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: shapes_materials.add(ShapesMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-25.0, 0.0, 0.0),
        ..default()
    });

    // inigo sd1 ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: sd1_materials.add(SD1Material {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(25.0, 0.0, 0.0),
        ..default()
    });

    // inigo sd2 ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: sd2_materials.add(SD2Material {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(50.0, 0.0, 0.0),
        ..default()
    });

    // inigo sd3 ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: sd3_materials.add(SD3Material {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(75.0, 0.0, 0.0),
        ..default()
    });

    // inigo sd4 ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: sd4_materials.add(SD4Material {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    });

    // -------------------------------------------------------------------------
    // row 3 -------------------------------------------------------------------
    // -------------------------------------------------------------------------

    // random static -----------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: random_static_materials.add(RandomStaticMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-100.0, -30.0, 0.0),
        ..default()
    });
}
