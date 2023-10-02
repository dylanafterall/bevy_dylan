pub mod components;
mod events;
mod systems;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CarryPlugin;

impl Plugin for CarryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::CastCarryRay>()
            .add_systems(
            Update,
            (
                systems::emit_cast_carry_ray,
                systems::handle_cast_carry_ray,
            ),
        );
    }
}
