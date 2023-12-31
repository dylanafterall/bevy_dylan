use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_hanabi::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_portal(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    const ORANGE_PORTAL_POS: Vec3 = Vec3::new(80.0, 0.0, 0.0);
    const BLUE_PORTAL_POS: Vec3 = Vec3::new(-80.0, 0.0, 0.0);

    // ORANGE PORTAL -----------------------------------------------------------
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(3.98, 1.25, 0.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(3.98, 1.25, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(3.98, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(3.98, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.3, Vec2::new(1.0, 0.1));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(0.0, 0.0, 0.0)).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(1.0).uniform(writer.lit(2.0)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let mut module = writer.finish();

    let mut tangent_accel =
        TangentAccelModifier::constant(&mut module, ORANGE_PORTAL_POS, Vec3::Z, 30.);

    let orange_portal = effects.add(
        EffectAsset::new(32768, Spawner::rate(5000.0.into()), module.clone())
            .with_name("orange_portal_effect")
            .init(init_pos)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(tangent_accel)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1.clone(),
                screen_space_size: false,
            })
            .render(OrientModifier {
                mode: OrientMode::AlongVelocity,
            }),
    );

    // BLUE PORTAL -------------------------------------------------------------
    color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(0.0, 1.25, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(0.0, 1.25, 4.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(0.0, 0.0, 4.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(0.0, 0.0, 4.0, 0.0));

    tangent_accel = TangentAccelModifier::constant(&mut module, BLUE_PORTAL_POS, Vec3::Z, 30.);

    let blue_portal = effects.add(
        EffectAsset::new(32768, Spawner::rate(5000.0.into()), module)
            .with_name("blue_portal_effect")
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
            .render(OrientModifier {
                mode: OrientMode::AlongVelocity,
            }),
    );

    // SPAWN BOTH PORTALS ------------------------------------------------------
    commands.spawn((
        Name::new("orange_portal"),
        RenderLayers::layer(1),
        ParticleEffectBundle {
            effect: ParticleEffect::new(orange_portal),
            transform: Transform::from_translation(ORANGE_PORTAL_POS),
            ..Default::default()
        },
    ));

    commands.spawn((
        Name::new("blue_portal"),
        RenderLayers::layer(1),
        ParticleEffectBundle {
            effect: ParticleEffect::new(blue_portal),
            transform: Transform::from_translation(BLUE_PORTAL_POS),
            ..Default::default()
        },
    ));
}

pub fn teleport_object() {}
