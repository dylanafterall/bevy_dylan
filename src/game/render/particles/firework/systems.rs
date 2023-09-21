use bevy::prelude::*;
use bevy_hanabi::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_firework(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.8));
    size_gradient1.add_key(0.3, Vec2::splat(0.8));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.0).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = writer.lit(Vec3::Y * -8.0).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(5.0).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        // axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(100.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        // axis: writer.lit(Vec3::Z).expr(),
        //speed: writer.lit(250.0).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(20.) + writer.lit(60.)).expr(),
    };

    let effect = EffectAsset::new(
        32768,
        Spawner::burst(2500.0.into(), 2.0.into()),
        writer.finish(),
    )
    .with_name("Firework")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .update(update_drag)
    .update(update_accel)
    .render(ColorOverLifetimeModifier {
        gradient: color_gradient1,
    })
    .render(SizeOverLifetimeModifier {
        gradient: size_gradient1,
        screen_space_size: false,
    });

    let effect1 = effects.add(effect);

    commands.spawn((
        Name::new("Firework"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
    ));
}
