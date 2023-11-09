use super::components::{Carrier, Carryable};
use super::events::CastCarryRay;
use crate::game::characters::player::components::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn emit_cast_carry_ray(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut event_cast: EventWriter<CastCarryRay>,
) {
    if keyboard_input.pressed(KeyCode::F) {
        event_cast.send(CastCarryRay);
        keyboard_input.reset(KeyCode::F);
    }
}

pub fn handle_cast_carry_ray(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &RapierRigidBodyHandle, &mut Carrier),
        With<Player>,
    >,
    carryable_query: Query<Entity, With<Carryable>>,
    mut rapier_context: ResMut<RapierContext>,
    mut event_listener: EventReader<CastCarryRay>,
) {
    for _ in event_listener.read() {
        let (player_entity, player_transform, player_rrbh, mut player_carrier) =
            player_query.single_mut();

        let ray_pos = player_transform.translation.truncate() + Vec2::new(7.6, 0.0);
        let ray_dir = Vec2::new(1.0, 0.0);
        let max_toi = 10.0;
        let solid = true;
        let filter = QueryFilter::default();

        if player_carrier.is_carrying == true {
            let player_body = player_rrbh.0;
            rapier_context
                .impulse_joints
                .remove_joints_attached_to_rigid_body(player_body);
            commands
                .entity(player_entity)
                .remove::<(ImpulseJoint, RapierImpulseJointHandle)>();
            player_carrier.is_carrying = false;
        } else if let Some((entity, _toi)) =
            rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            // let hit_point = ray_pos + ray_dir * toi;
            match carryable_query.get(entity) {
                Ok(carryable_entity) => {
                    if player_carrier.is_carrying == false {
                        let joint = RopeJointBuilder::new().limits([0.0, 15.0]);
                        commands
                            .entity(player_entity)
                            .insert(ImpulseJoint::new(carryable_entity, joint));
                        player_carrier.is_carrying = true;
                    }
                }
                Err(_) => {}
            }
        }
    }
}
