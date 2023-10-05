use super::components::*;
use crate::game::characters::components::*;

use crate::game::characters::friendly::bloom_triangle::events::BloomTriangleContact;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_bloom_triangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("Bloom"))
        .insert(NPC::BloomTriangle)
        .insert(BloomColor::Magenta)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::triangle(
            Vec2::new(0.0, 10.0),
            Vec2::new(-8.66, -5.0),
            Vec2::new(8.66, -5.0),
        ))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.2,
        })
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(10.0, 3).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(75.0, 25.0, 0.0)));
}

pub fn handle_player_triangle_contact(
    mut commands: Commands,
    mut triangle_contact_listener: EventReader<BloomTriangleContact>,
    mut bloom_query: Query<(Entity, &mut BloomColor)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in triangle_contact_listener.iter() {
        let (triangle, mut bloom) = bloom_query.single_mut();
        match *bloom {
            BloomColor::Red => {
                *bloom = BloomColor::Blue;
                let new_color = ColorMaterial::from(Color::rgb(0.0, 0.0, 4.0));
                let handle = materials.add(new_color);
                commands.entity(triangle).insert(handle);
            }
            BloomColor::Blue => {
                *bloom = BloomColor::Green;
                let new_color = ColorMaterial::from(Color::rgb(0.0, 4.0, 0.0));
                let handle = materials.add(new_color);
                commands.entity(triangle).insert(handle);
            }
            BloomColor::Green => {
                *bloom = BloomColor::Magenta;
                let new_color = ColorMaterial::from(Color::rgb(4.0, 0.0, 4.0));
                let handle = materials.add(new_color);
                commands.entity(triangle).insert(handle);
            }
            BloomColor::Magenta => {
                *bloom = BloomColor::Cyan;
                let new_color = ColorMaterial::from(Color::rgb(0.0, 4.0, 4.0));
                let handle = materials.add(new_color);
                commands.entity(triangle).insert(handle);
            }
            BloomColor::Cyan => {
                *bloom = BloomColor::Yellow;
                let new_color = ColorMaterial::from(Color::rgb(4.0, 4.0, 0.0));
                let handle = materials.add(new_color);
                commands.entity(triangle).insert(handle);
            }
            BloomColor::Yellow => {
                *bloom = BloomColor::Red;
                let new_color = ColorMaterial::from(Color::rgb(4.0, 0.0, 0.0));
                let handle = materials.add(new_color);
                commands.entity(triangle).insert(handle);
            }
        };
    }
}
