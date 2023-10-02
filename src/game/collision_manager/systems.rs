use super::components::*;
use super::events::*;
use crate::game::characters::player::components::*;
use crate::game::objects::joints::components::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn parse_collision_event(
    player_query: Query<&Player>,
    sensor_query: Query<&PlayerCollisionSensor>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_scene_collision: EventWriter<PlayerSceneCollision>,
    mut player_grav_collision: EventWriter<PlayerGravCollision>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _collision_flag) => {
                if player_query.get(*entity1).is_ok() {
                    // if entity 1 is the player, check if entity 2 is a relevant sensor
                    let _sensor_result = match sensor_query.get(*entity2) {
                        // if entity 2 IS a relevant sensor
                        Ok(sensor_variant) => {
                            match sensor_variant {
                                // if the player has entered a sensor to change the scene
                                PlayerCollisionSensor::SceneSensor(scene_state) => {
                                    player_scene_collision.send(PlayerSceneCollision {
                                        desired_scene: *scene_state,
                                    });
                                }
                                // if the player has entered a sensor to change gravity direction
                                PlayerCollisionSensor::GravSensor(grav_dir) => {
                                    player_grav_collision.send(PlayerGravCollision {
                                        grav_direction: *grav_dir,
                                    });
                                }
                            }
                        }
                        // if entity 2 IS NOT a relevant sensor
                        Err(_e) => {}
                    };
                } else if player_query.get(*entity2).is_ok() {
                    // if entity 2 is the player, check if entity 1 is a relevant sensor
                    let _sensor_result = match sensor_query.get(*entity1) {
                        // if entity 1 IS a relevant sensor
                        Ok(sensor_variant) => {
                            match sensor_variant {
                                // if the player has entered a sensor to change the scene
                                PlayerCollisionSensor::SceneSensor(scene_state) => {
                                    player_scene_collision.send(PlayerSceneCollision {
                                        desired_scene: *scene_state,
                                    });
                                }
                                // if the player has entered a sensor to change gravity direction
                                PlayerCollisionSensor::GravSensor(grav_dir) => {
                                    player_grav_collision.send(PlayerGravCollision {
                                        grav_direction: *grav_dir,
                                    });
                                }
                            }
                        }
                        // if entity 1 IS NOT a relevant sensor
                        Err(_e) => {}
                    };
                }
            }
            CollisionEvent::Stopped(_handle1, _handle2, _flag) => {}
        }
    }
}

pub fn parse_contact_force_event(
    player_query: Query<&Player>,
    destructible_query: Query<&RapierRigidBodyHandle, With<Destructible>>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut player_contact: EventWriter<PlayerContact>,
    mut destructible_contact: EventWriter<DestructibleContact>,
) {
    for contact_force_event in contact_force_events.iter() {
        let entity1 = contact_force_event.collider1;
        let entity2 = contact_force_event.collider2;
        let total_force = contact_force_event.total_force;

        // check for player contact events
        if player_query.get(entity1).is_ok() {
            player_contact.send(PlayerContact {
                player: entity1,
                partner: entity2,
                force_vector: total_force,
            });
        } else if player_query.get(entity2).is_ok() {
            player_contact.send(PlayerContact {
                player: entity2,
                partner: entity1,
                force_vector: total_force,
            });
        }

        // check for destructible contact events (could be either or both entities)
        let _rigid_body_result = match destructible_query.get(entity1) {
            Ok(rb_handle) => {
                destructible_contact.send(DestructibleContact {
                    destructible_rb_handle: *rb_handle,
                    destructible_entity: entity1,
                });
            }
            Err(_e) => {}
        };
        let _rigid_body_result = match destructible_query.get(entity2) {
            Ok(rb_handle) => {
                destructible_contact.send(DestructibleContact {
                    destructible_rb_handle: *rb_handle,
                    destructible_entity: entity2,
                });
            }
            Err(_e) => {}
        };
    }
}
