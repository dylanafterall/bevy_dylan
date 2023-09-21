use super::components::*;

use crate::style::FRAPPE_SURFACE2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

// -----------------------------------------------------------------------------
pub fn spawn_elevator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("Elevator"))
        .insert(ElevatorTimer {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            heading_up: true,
        })
        // physics
        // -------
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(20.0, 2.5))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 5.0),
            angvel: 0.0,
        })
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(40.0, 5.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(60.0, -20.0, 0.0)));
}

pub fn move_elevator(
    mut elevator_query: Query<(&mut ElevatorTimer, &mut Velocity)>,
    time: Res<Time>,
) {
    for (mut ele_timer, mut ele_velocity) in elevator_query.iter_mut() {
        ele_timer.timer.tick(time.delta());

        if ele_timer.timer.finished() && ele_timer.heading_up {
            ele_velocity.linvel = Vec2::new(0.0, -5.0);
            ele_timer.heading_up = !ele_timer.heading_up;
        } else if ele_timer.timer.finished() && !ele_timer.heading_up {
            ele_velocity.linvel = Vec2::new(0.0, 5.0);
            ele_timer.heading_up = !ele_timer.heading_up;
        }
    }
}
