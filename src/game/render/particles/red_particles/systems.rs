use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_red_particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 1.0, 1.0));

    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::new(-400.0, 400.0, 0.0)),
        axis: module.lit(Vec3::Z),
        radius: module.lit(50.0),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::new(-400.0, 400.0, 0.0)),
        axis: module.lit(Vec3::Z),
        speed: module.lit(10.0),
    };

    let lifetime = module.lit(10.0);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = module.lit(Vec3::new(0.0, -6.0, 0.0));
    let update_accel = AccelModifier::new(accel);

    let effect = EffectAsset::new(32768, Spawner::rate(5.0.into()), module)
        .with_name("RedEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient });

    let effect_handle = effects.add(effect);

    // Configure the emitter to spawn 100 particles / second
    let spawner = Spawner::rate(100.0.into());

    commands.spawn((
        Name::new("RedParticles"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle).with_spawner(spawner),
            transform: Transform::from_translation(Vec3::Y),
            ..Default::default()
        },
    ));
}
