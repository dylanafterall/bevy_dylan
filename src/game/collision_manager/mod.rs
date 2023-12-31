pub mod components;
pub mod events;
mod systems;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CollisionManagerPlugin;

impl Plugin for CollisionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_event::<events::PlayerSceneCollision>()
            .add_event::<events::PlayerGravCollision>()
            .add_event::<events::PlayerContact>()
            .add_event::<events::DestructibleContact>()
            .add_systems(
                Update,
                (
                    systems::parse_collision_event,
                    systems::parse_contact_force_event,
                ),
            );
    }
}
