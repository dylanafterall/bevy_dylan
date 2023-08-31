pub mod events;
mod systems;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CollisionManagerPlugin;

impl Plugin for CollisionManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_event::<events::PlayerSceneCollision>()
            .add_event::<events::PlayerOtherCollision>()
            .add_event::<events::PlayerCharacterCollision>()

            .add_systems(Update, (
                systems::parse_collision_event,
                systems::parse_contact_force_event,
            ));
    }
}