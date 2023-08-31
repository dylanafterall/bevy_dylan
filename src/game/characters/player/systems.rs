use crate::game::collision_manager::events::*;
use super::components::*;
use super::events::*;

use bevy::prelude::*;
use bevy::render::view::visibility::RenderLayers;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(Player)
        .insert(RenderLayers::layer(3))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(200.0))
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule(Vec2::new(0.0, -25.0), Vec2::new(0.0, 25.0), 50.0))
        .insert(Ccd::enabled())
        .insert(Dominance::group(0))
        .insert(Restitution::coefficient(0.7))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(GravityScale(0.0))
        .insert(Damping { linear_damping: 1.0, angular_damping: 2.0 })
        // .insert(CollisionGroups::new(0b1101.into(), 0b0100.into())
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(Friction {
            coefficient: 0.7,
            combine_rule: CoefficientCombineRule::Average,
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        });
}

// -----------------------------------------------------------------------------
pub fn emit_player_move_up(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveUp>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        event_source.send(PlayerMoveUp {});
        keyboard_input.reset(KeyCode::W);
    }
}

pub fn emit_player_move_down(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveDown>,
) {
    if keyboard_input.pressed(KeyCode::S) {
        event_source.send(PlayerMoveDown {});
        keyboard_input.reset(KeyCode::S);
    }
}

pub fn emit_player_move_left(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveLeft>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        event_source.send(PlayerMoveLeft {});
        keyboard_input.reset(KeyCode::A);
    }
}

pub fn emit_player_move_right(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveRight>,
) {
    if keyboard_input.pressed(KeyCode::D) {
        event_source.send(PlayerMoveRight {});
        keyboard_input.reset(KeyCode::D);
    }
}

// -----------------------------------------------------------------------------
pub fn handle_player_move_up(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveUp>,
) {
    for _ in event_listener.iter() {
        for mut ext_impulse in impulse_query.iter_mut() {
            ext_impulse.impulse = Vec2::new(0.0, 50.0);
            ext_impulse.torque_impulse = 0.0;
        }
    }
}

pub fn handle_player_move_down(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveDown>,
) {
    for _ in event_listener.iter() {
        for mut ext_impulse in impulse_query.iter_mut() {
            ext_impulse.impulse = Vec2::new(0.0, -50.0);
            ext_impulse.torque_impulse = 0.0;
        }
    }
}

pub fn handle_player_move_left(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveLeft>,
) {
    for _ in event_listener.iter() {
        for mut ext_impulse in impulse_query.iter_mut() {
            ext_impulse.impulse = Vec2::new(-50.0, 0.0);
            ext_impulse.torque_impulse = 0.0;
        }
    }
}

pub fn handle_player_move_right(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveRight>,
) {
    for _ in event_listener.iter() {
        for mut ext_impulse in impulse_query.iter_mut() {
            ext_impulse.impulse = Vec2::new(50.0, 0.0);
            ext_impulse.torque_impulse = 0.0;
        }
    }
}

pub fn handle_player_character_collision(
    mut friendly_contact_listener: EventReader<PlayerFriendlyContact>,
    mut hostile_contact_listener: EventReader<PlayerHostileContact>,
) {
    for friendly_contact in friendly_contact_listener.iter() {
        println!(
            "Friendly contact between: {:?}, {:?}, {:?}",
            friendly_contact.player,
            friendly_contact.partner,
            friendly_contact.force_vector,
        );
    }
    for hostile_contact in hostile_contact_listener.iter() {
        println!(
            "Hostile contact between: {:?}, {:?}, {:?}",
            hostile_contact.player,
            hostile_contact.partner,
            hostile_contact.force_vector,
        );
    }
}
