use crate::game::collision_manager::components::*;
use crate::game::collision_manager::events::PlayerGravCollision;
use crate::GRAV_MAG;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_grav_shift(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_up = asset_server.load("images/arrows/up.png");
    let texture_right = asset_server.load("images/arrows/right.png");
    let texture_down = asset_server.load("images/arrows/down.png");
    let texture_left = asset_server.load("images/arrows/left.png");

    // up ----------------------------------------------------------------------
    commands
        // info
        // ----
        .spawn(Name::new("GravShiftUp"))
        .insert(PlayerCollisionSensor::GravSensor(Vec2::new(0.0, GRAV_MAG)))
        // physics
        // -------
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_up)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-70.0, 25.0, 0.0)));

    // right --------------------------------------------------------------------
    commands
        // info
        // ----
        .spawn(Name::new("GravShiftRight"))
        .insert(PlayerCollisionSensor::GravSensor(Vec2::new(GRAV_MAG, 0.0)))
        // physics
        // -------
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_right)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-60.0, 15.0, 0.0)));

    // down --------------------------------------------------------------------
    commands
        // info
        // ----
        .spawn(Name::new("GravShiftDown"))
        .insert(PlayerCollisionSensor::GravSensor(Vec2::new(0.0, -GRAV_MAG)))
        // physics
        // -------
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_down)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-70.0, 5.0, 0.0)));

    // left --------------------------------------------------------------------
    commands
        // info
        // ----
        .spawn(Name::new("GravShiftLeft"))
        .insert(PlayerCollisionSensor::GravSensor(Vec2::new(-GRAV_MAG, 0.0)))
        // physics
        // -------
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_left)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-80.0, 15.0, 0.0)));
}

pub fn handle_grav_shift(
    mut rapier: ResMut<RapierConfiguration>,
    mut event_listener: EventReader<PlayerGravCollision>,
) {
    for grav_collision in event_listener.read() {
        let grav_dir = grav_collision.grav_direction;
        rapier.gravity = grav_dir;
    }
}

pub fn reset_grav_shift(mut rapier: ResMut<RapierConfiguration>) {
    rapier.gravity = Vec2::new(0.0, -GRAV_MAG);
}
