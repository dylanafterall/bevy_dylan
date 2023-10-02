use super::super::components::*;
use super::components::*;
use super::events::*;
use crate::game::collision_manager::events::*;

use crate::game::characters::player::carry::components::Carrier;
use crate::style::FRAPPE_TEXT;
use bevy::{prelude::*, render::view::visibility::RenderLayers, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut capsule_shape = shape::Capsule::default();
    capsule_shape.radius = 5.0;
    capsule_shape.depth = 5.0;

    commands
        // info
        // ----
        .spawn(Name::new("Player"))
        .insert(Player)
        .insert(Carrier { is_carrying: false })
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule(
            Vec2::new(0.0, -2.5),
            Vec2::new(0.0, 2.5),
            5.0,
        ))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(0.5))
        .insert(Dominance::group(0))
        .insert(Restitution::coefficient(0.7))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 2.0,
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
        })
        // .insert(CollisionGroups::new(0b1101.into(), 0b0100.into())
        // rendering
        // ---------
        .insert(RenderLayers::layer(3))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(capsule_shape.into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_TEXT)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

// -----------------------------------------------------------------------------
pub fn emit_player_move_up(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveUp>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        event_source.send(PlayerMoveUp);
        keyboard_input.reset(KeyCode::W);
    }
}

pub fn emit_player_move_down(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveDown>,
) {
    if keyboard_input.pressed(KeyCode::S) {
        event_source.send(PlayerMoveDown);
        keyboard_input.reset(KeyCode::S);
    }
}

pub fn emit_player_move_left(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveLeft>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        event_source.send(PlayerMoveLeft);
        keyboard_input.reset(KeyCode::A);
    }
}

pub fn emit_player_move_right(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_source: EventWriter<PlayerMoveRight>,
) {
    if keyboard_input.pressed(KeyCode::D) {
        event_source.send(PlayerMoveRight);
        keyboard_input.reset(KeyCode::D);
    }
}

// -----------------------------------------------------------------------------
pub fn handle_player_move_up(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveUp>,
) {
    let mut player = impulse_query.single_mut();

    for _ in event_listener.iter() {
        player.impulse = Vec2::new(0.0, 0.1);
        player.torque_impulse = 0.0;
    }
}

pub fn handle_player_move_down(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveDown>,
) {
    let mut player = impulse_query.single_mut();

    for _ in event_listener.iter() {
        player.impulse = Vec2::new(0.0, -0.1);
        player.torque_impulse = 0.0;
    }
}

pub fn handle_player_move_left(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveLeft>,
) {
    let mut player = impulse_query.single_mut();

    for _ in event_listener.iter() {
        player.impulse = Vec2::new(-0.1, 0.0);
        player.torque_impulse = 0.0;
    }
}

pub fn handle_player_move_right(
    mut impulse_query: Query<&mut ExternalImpulse, With<Player>>,
    mut event_listener: EventReader<PlayerMoveRight>,
) {
    let mut player = impulse_query.single_mut();

    for _ in event_listener.iter() {
        player.impulse = Vec2::new(0.1, 0.0);
        player.torque_impulse = 0.0;
    }
}

pub fn handle_player_character_collision(
    npc_query: Query<&NPC>,
    mut player_contact_listener: EventReader<PlayerContact>,
) {
    for player_contact in player_contact_listener.iter() {
        let partner = player_contact.partner;

        let _npc_result = match npc_query.get(partner) {
            Ok(npc) => match npc {
                NPC::Friendly => {
                    println!(
                        "Friendly contact between: {:?}, {:?}, {:?}",
                        player_contact.player, partner, player_contact.force_vector,
                    );
                }
                NPC::Neutral => {
                    println!(
                        "Neutral contact between: {:?}, {:?}, {:?}",
                        player_contact.player, partner, player_contact.force_vector,
                    );
                }
                NPC::Hostile => {
                    println!(
                        "Hostile contact between: {:?}, {:?}, {:?}",
                        player_contact.player, partner, player_contact.force_vector,
                    );
                }
            },
            Err(_e) => {}
        };
    }
}
