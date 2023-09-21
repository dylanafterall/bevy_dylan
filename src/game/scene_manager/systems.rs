use super::SceneState;
use crate::game::collision_manager::components::*;
use crate::game::collision_manager::events::PlayerSceneCollision;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_scene_sensors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle1 = asset_server.load("images/numerals/1.png");
    let texture_handle2 = asset_server.load("images/numerals/2.png");
    let texture_handle3 = asset_server.load("images/numerals/3.png");
    let texture_handle4 = asset_server.load("images/numerals/4.png");
    let texture_handle5 = asset_server.load("images/numerals/5.png");

    commands
        // info
        // ----
        .spawn(Name::new("Scene1Sensor"))
        .insert(PlayerCollisionSensor::SceneSensor(SceneState::First))
        // physics
        // -------
        .insert(Collider::cuboid(7.5, 7.5))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(15.0, 15.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_handle1)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -100.0, 60.0, 0.0,
        )));

    commands
        // info
        // ----
        .spawn(Name::new("Scene2Sensor"))
        .insert(PlayerCollisionSensor::SceneSensor(SceneState::Second))
        // physics
        // -------
        .insert(Collider::cuboid(7.5, 7.5))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(15.0, 15.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_handle2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-50.0, 60.0, 0.0)));

    commands
        // info
        // ----
        .spawn(Name::new("Scene3Sensor"))
        .insert(PlayerCollisionSensor::SceneSensor(SceneState::Third))
        // physics
        // -------
        .insert(Collider::cuboid(7.5, 7.5))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(15.0, 15.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_handle3)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 60.0, 0.0)));

    commands
        // info
        // ----
        .spawn(Name::new("Scene4Sensor"))
        .insert(PlayerCollisionSensor::SceneSensor(SceneState::Fourth))
        // physics
        // -------
        .insert(Collider::cuboid(7.5, 7.5))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(15.0, 15.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_handle4)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(50.0, 60.0, 0.0)));

    commands
        // info
        // ----
        .spawn(Name::new("Scene5Sensor"))
        .insert(PlayerCollisionSensor::SceneSensor(SceneState::Fifth))
        // physics
        // -------
        .insert(Collider::cuboid(7.5, 7.5))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(15.0, 15.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(texture_handle5)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(100.0, 60.0, 0.0)));
}

// -----------------------------------------------------------------------------
pub fn despawn_entities(
    mut commands: Commands,
    collider_query: Query<Option<Entity>, With<Collider>>,
    mesh_query: Query<Option<Entity>, (With<Mesh2dHandle>, Without<Collider>)>,
    particle_query: Query<Option<Entity>, With<ParticleEffect>>,
) {
    for collider in collider_query.iter() {
        match collider {
            Some(c_entity) => commands.entity(c_entity).despawn(),
            None => {}
        }
    }

    for mesh in mesh_query.iter() {
        match mesh {
            Some(m_entity) => commands.entity(m_entity).despawn(),
            None => {}
        }
    }

    for particles in particle_query.iter() {
        match particles {
            Some(p_entity) => commands.entity(p_entity).despawn(),
            None => {}
        }
    }
}

pub fn handle_scene_transition(
    mut next_scene_state: ResMut<NextState<SceneState>>,
    mut event_listener: EventReader<PlayerSceneCollision>,
) {
    for scene_collision in event_listener.iter() {
        let next_scene = scene_collision.desired_scene;
        next_scene_state.set(next_scene);
    }
}
