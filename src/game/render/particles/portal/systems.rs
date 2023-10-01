use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_portal(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.3, Vec2::new(0.2, 0.02));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(80.0, 0.0, 0.0)).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.6).uniform(writer.lit(1.3)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let mut module = writer.finish();

    let tangent_accel = TangentAccelModifier::constant(&mut module, Vec3::ZERO, Vec3::Z, 30.);

    let effect1 = effects.add(
        EffectAsset::new(32768, Spawner::rate(5000.0.into()), module)
            .with_name("portal")
            .init(init_pos)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(tangent_accel)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            })
            .render(OrientAlongVelocityModifier),
    );

    commands.spawn((
        Name::new("portal"),
        RenderLayers::layer(2),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform: Transform::IDENTITY,
            ..Default::default()
        },
    ));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(10.0))
        .insert(TransformBundle::from(Transform::from_xyz(80.0, 0.0, 0.0)));
}
