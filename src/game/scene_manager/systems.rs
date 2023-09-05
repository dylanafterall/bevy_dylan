use super::SceneState;
use super::components::*;

use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;
use crate::game::collision_manager::events::PlayerSceneCollision;

// -----------------------------------------------------------------------------
pub fn spawn_first_scene_sensors(
    mut commands: Commands,
) {
    commands
        .spawn(SceneSensor { desired_scene: SceneState::Third})
        .insert(Collider::cuboid(100.0, 100.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TransformBundle::from(Transform::from_xyz(-1000.0, 550.0, 0.0)));
    commands
        .spawn(SceneSensor{ desired_scene: SceneState::Second})
        .insert(Collider::cuboid(100.0, 100.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TransformBundle::from(Transform::from_xyz(1000.0, 550.0, 0.0)));
}

pub fn spawn_second_scene_sensors(
    mut commands: Commands,
) {
    commands
        .spawn(SceneSensor { desired_scene: SceneState::First})
        .insert(Collider::cuboid(100.0, 100.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TransformBundle::from(Transform::from_xyz(-1000.0, 0.0, 0.0)));
    commands
        .spawn(SceneSensor{ desired_scene: SceneState::Third})
        .insert(Collider::cuboid(100.0, 100.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TransformBundle::from(Transform::from_xyz(1000.0, 0.0, 0.0)));
}

pub fn spawn_third_scene_sensors(
    mut commands: Commands,
) {
    commands
        .spawn(SceneSensor { desired_scene: SceneState::Second})
        .insert(Collider::cuboid(100.0, 100.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TransformBundle::from(Transform::from_xyz(-1000.0, -550.0, 0.0)));
    commands
        .spawn(SceneSensor{ desired_scene: SceneState::First})
        .insert(Collider::cuboid(100.0, 100.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TransformBundle::from(Transform::from_xyz(1000.0, -550.0, 0.0)));
}

pub fn despawn_entities(
    mut commands: Commands,
    collider_query: Query<Entity, With<Collider>>,
    shape_query: Query<Entity, With<Mesh2dHandle>>,
) {
    for collider in collider_query.iter() {
        commands.entity(collider).despawn();
    }
    for shape in shape_query.iter() {
        commands.entity(shape).despawn();
    }
}

pub fn handle_scene_transition(
    mut next_scene_state: ResMut<NextState<SceneState>>,
    mut event_listener: EventReader<PlayerSceneCollision>,
    scene_sensor_query: Query<&SceneSensor>,
) {
    for scene_collision in event_listener.iter() {
        let sensor = scene_sensor_query.get(scene_collision.scene_sensor).unwrap();
        let next_scene = sensor.desired_scene;

        match next_scene {
            SceneState::Inert => {}
            SceneState::First => {
                next_scene_state.set(SceneState::First);
                println!("Entered first scene");
            }
            SceneState::Second => {
                next_scene_state.set(SceneState::Second);
                println!("Entered second scene");
            }
            SceneState::Third => {
                next_scene_state.set(SceneState::Third);
                println!("Entered third scene");
            }
        }
    }
}