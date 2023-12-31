use crate::game::characters::player::components::Player;
use crate::game::render::particles::force_field::components::ForceFieldParticles;
use crate::style::{FRAPPE_CRUST, LATTE_BLUE, LATTE_RED};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_hanabi::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_force_field(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const BALL_RADIUS: f32 = 5.0;

    let attractor1_position = Vec3::new(0.0, 30.0, 0.0);
    let attractor2_position = Vec3::new(0.0, -30.0, 0.0);

    // attractor1
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(LATTE_BLUE)),
            ..default()
        })
        .insert(Transform::from_translation(attractor1_position));

    // attractor2
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(LATTE_RED)),
            ..default()
        })
        .insert(Transform::from_translation(attractor2_position));

    // "forbid" circle
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(12.0).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_CRUST)),
            ..default()
        })
        .insert(Transform::from_xyz(-30.0, 0.0, 0.0));

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 1.0, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 1.0, 1.0, 0.0));

    // Prevent the spawner from immediately spawning on activation, and instead
    // require a manual reset() call.
    let spawn_immediately = false;

    let spawner = Spawner::once(30.0.into(), spawn_immediately);

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(8.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let center = writer.lit(Vec3::ZERO).expr();
    let half_size = writer.lit(Vec3::new(75.0, 50.0, 12.0)).expr();
    let allow_zone = KillAabbModifier::new(center, half_size);

    let center = writer.lit(Vec3::new(-30.0, 0.0, 0.0)).expr();
    let radius = writer.lit(12.0);
    let sqr_radius = (radius.clone() * radius).expr();
    let deny_zone = KillSphereModifier::new(center, sqr_radius).with_kill_inside(true);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(BALL_RADIUS).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(2.0) + writer.lit(1.0)).expr(),
    };

    // Force field effects
    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .with_name("force_field")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(ForceFieldModifier::new(vec![
                // repulsive force field
                ForceFieldSource {
                    position: attractor2_position,
                    max_radius: 300.0,
                    min_radius: 1.0,
                    mass: -150.0,
                    // linear force: proportional to 1 / distance
                    force_exponent: 1.0,
                    conform_to_sphere: true,
                },
                // attractive force field
                ForceFieldSource {
                    position: attractor1_position,
                    max_radius: 300.0,
                    min_radius: 8.5,
                    mass: 1000.0,
                    // quadratic force: proportional to 1 / distance^2
                    force_exponent: 2.0,
                    conform_to_sphere: true,
                },
            ]))
            .update(allow_zone)
            .update(deny_zone)
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(1.0)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .spawn(ParticleEffectBundle::new(effect).with_spawner(spawner))
        .insert(ForceFieldParticles)
        .insert(RenderLayers::layer(1));
}

pub fn update(
    mut q_effect: Query<
        (&mut EffectSpawner, &mut Transform),
        (With<ForceFieldParticles>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // CoreSet::PostUpdate, so will not be available yet. Ignore for a frame if
    // so.
    let Ok((mut spawner, mut effect_transform)) = q_effect.get_single_mut() else {
        return;
    };

    let player_transform = player_query.single();
    if keyboard_input.pressed(KeyCode::P) {
        effect_transform.translation = player_transform.translation;
        // Spawn a single burst of particles
        spawner.reset();
        keyboard_input.reset(KeyCode::P);
    }
}
