use crate::game::scene_manager::components::*;
use crate::game::characters::player::components::*;
use crate::game::objects::joints::components::*;
use super::events::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn parse_collision_event(
    player_query: Query<&Player>,
    scene_sensor_query: Query<&SceneSensor>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_scene_collision: EventWriter<PlayerSceneCollision>,
    mut player_other_collision: EventWriter<PlayerOtherCollision>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, collision_flag) => {
                if player_query.get(*entity1).is_ok() {
                    if scene_sensor_query.get(*entity2).is_ok() {
                        player_scene_collision.send(PlayerSceneCollision {
                            scene_sensor: *entity2,
                        });
                    }
                    else {
                        player_other_collision.send(PlayerOtherCollision {
                            player: *entity1,
                            partner: *entity2,
                            flag: *collision_flag,
                        });
                    }
                }
                else if player_query.get(*entity2).is_ok() {
                    if scene_sensor_query.get(*entity1).is_ok() {
                        player_scene_collision.send(PlayerSceneCollision {
                            scene_sensor: *entity1,
                        });
                    }
                    else {
                        player_other_collision.send(PlayerOtherCollision {
                            player: *entity2,
                            partner: *entity1,
                            flag: *collision_flag,
                        });
                    }
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
        }
        else if player_query.get(entity2).is_ok() {
            player_contact.send(PlayerContact {
                player: entity2,
                partner: entity1,
                force_vector: total_force,
            });
        }

        // check for destructible contact events
        if destructible_query.get(entity1).is_ok() {
            let rigid_body = destructible_query.get(entity1).unwrap();
            destructible_contact.send(DestructibleContact {
                destructible: *rigid_body,
            });
        }
        else if destructible_query.get(entity2).is_ok() {
            let rigid_body = destructible_query.get(entity2).unwrap();
            destructible_contact.send(DestructibleContact {
                destructible: *rigid_body,
            });
        }
    }
}
