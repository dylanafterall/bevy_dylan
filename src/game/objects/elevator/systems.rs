use super::components::*;

use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_elevator(
    mut commands: Commands,
) {
    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(ElevatorTimer {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            heading_up: true,
        })
        .insert(Collider::cuboid(200.0, 25.0))
        .insert(TransformBundle::from(Transform::from_xyz(600.0, -200.0, 0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 100.0),
            angvel: 0.0,
        });
}

pub fn move_elevator(
    mut elevator_query: Query<(&mut ElevatorTimer, &mut Velocity)>,
    time: Res<Time>,
) {
    for (mut ele_timer, mut ele_velocity) in elevator_query.iter_mut() {
        ele_timer.timer.tick(time.delta());

        if ele_timer.timer.finished() && ele_timer.heading_up {
            ele_velocity.linvel = Vec2::new(0.0, -100.0);
            ele_timer.heading_up = !ele_timer.heading_up;
        }
        else if ele_timer.timer.finished() && !ele_timer.heading_up {
            ele_velocity.linvel = Vec2::new(0.0, 100.0);
            ele_timer.heading_up = !ele_timer.heading_up;
        }
    }
}