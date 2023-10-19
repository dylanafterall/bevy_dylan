use crate::materials::{
    blue_flash_material::BlueFlashMaterial, color_blend_material::ColorBlendMaterial,
    color_grid_material::ColorGridMaterial, color_polar_material::ColorPolarMaterial,
    easing::EasingMaterial, rainbow_material::RainbowMaterial, random_static::RandomStaticMaterial,
    red_flash_material::RedFlashMaterial, red_flash_smooth_material::RedFlashSmoothMaterial,
    red_flash_tangent_material::RedFlashTangentMaterial, red_material::RedMaterial,
    sd1_material::SD1Material, sd2_material::SD2Material, sd3_material::SD3Material,
    sd4_material::SD4Material, shape_collision_material::ShapeCollisionMaterial,
    shape_morph_material::ShapeMorphMaterial, shapes_material::ShapesMaterial,
    swirly_material::SwirlyMaterial,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// -----------------------------------------------------------------------------
// row 1 -----------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_first_row_shaders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut red_materials: ResMut<Assets<RedMaterial>>,
    mut red_flash_materials: ResMut<Assets<RedFlashMaterial>>,
    mut red_flash_tangent_materials: ResMut<Assets<RedFlashTangentMaterial>>,
    mut red_flash_smooth_materials: ResMut<Assets<RedFlashSmoothMaterial>>,
    mut blue_flash_materials: ResMut<Assets<BlueFlashMaterial>>,
    mut easing_materials: ResMut<Assets<EasingMaterial>>,
    mut rainbow_materials: ResMut<Assets<RainbowMaterial>>,
    mut polar_materials: ResMut<Assets<ColorPolarMaterial>>,
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

    // rainbow -----------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: rainbow_materials.add(RainbowMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(75.0, 30.0, 0.0),
        ..default()
    });

    // polar coordinates -------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: polar_materials.add(ColorPolarMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(100.0, 30.0, 0.0),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// row 2 -----------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_second_row_shaders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut shapes_materials: ResMut<Assets<ShapesMaterial>>,
    mut sd1_materials: ResMut<Assets<SD1Material>>,
    mut sd2_materials: ResMut<Assets<SD2Material>>,
    mut sd3_materials: ResMut<Assets<SD3Material>>,
    mut sd4_materials: ResMut<Assets<SD4Material>>,
    mut shape_morph_materials: ResMut<Assets<ShapeMorphMaterial>>,
    mut shape_collision_materials: ResMut<Assets<ShapeCollisionMaterial>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/white_square.png");

    // inigo sd1 ---------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: sd1_materials.add(SD1Material {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-100.0, 0.0, 0.0),
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
        transform: Transform::from_xyz(-75.0, 0.0, 0.0),
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
        transform: Transform::from_xyz(-50.0, 0.0, 0.0),
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
        transform: Transform::from_xyz(-25.0, 0.0, 0.0),
        ..default()
    });

    // simpler shapes ----------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: shapes_materials.add(ShapesMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(25.0, 0.0, 0.0),
        ..default()
    });

    // shape morph -------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: shape_morph_materials.add(ShapeMorphMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(50.0, 0.0, 0.0),
        ..default()
    });

    // shape collision ---------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: shape_collision_materials.add(ShapeCollisionMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(75.0, 0.0, 0.0),
        ..default()
    });
}

// -----------------------------------------------------------------------------
// row 3 -----------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_third_row_shaders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut random_static_materials: ResMut<Assets<RandomStaticMaterial>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/white_square.png");

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

// -----------------------------------------------------------------------------
// row 4 -----------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_fourth_row_shaders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut color_grid_materials: ResMut<Assets<ColorGridMaterial>>,
    mut color_blend_materials: ResMut<Assets<ColorBlendMaterial>>,
    mut swirly_materials: ResMut<Assets<SwirlyMaterial>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/white_square.png");

    // color grid (triangles) --------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: color_grid_materials.add(ColorGridMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-100.0, -60.0, 0.0),
        ..default()
    });

    // color blend -------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: color_blend_materials.add(ColorBlendMaterial {}),
        transform: Transform::from_xyz(-75.0, -60.0, 0.0),
        ..default()
    });

    // swirly ------------------------------------------------------------------
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(20.0, 20.0))))
            .into(),
        material: swirly_materials.add(SwirlyMaterial {
            color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
        }),
        transform: Transform::from_xyz(-50.0, -60.0, 0.0),
        ..default()
    });
}
