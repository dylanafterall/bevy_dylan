use crate::game::render::materials::NeatMaterial;
use crate::style::{LATTE_LAVENDER, LATTE_YELLOW};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

// -----------------------------------------------------------------------------
pub fn spawn_shader_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NeatMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("images/bevy_icon.png");

    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10.0, 10.0)))).into(),
            material: materials.add(NeatMaterial {
                color: Color::from(LATTE_YELLOW),
                color_texture: Some(texture_handle.clone()),
                alpha_mode: AlphaMode::Blend,
            }),
            transform: Transform::from_xyz(-40.0, 0.0, 0.0),
            ..default()
        })
        .insert(RenderLayers::layer(1));

    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(10.0, 10.0)))).into(),
            material: materials.add(NeatMaterial {
                color: Color::from(LATTE_LAVENDER),
                color_texture: Some(texture_handle),
                alpha_mode: AlphaMode::Blend,
            }),
            transform: Transform::from_xyz(40.0, 0.0, 0.0),
            ..default()
        })
        .insert(RenderLayers::layer(1));
}
