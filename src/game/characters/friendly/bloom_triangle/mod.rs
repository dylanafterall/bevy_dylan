mod components;
pub mod events;
mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct BloomTrianglePlugin;

impl Plugin for BloomTrianglePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_event::<events::BloomTriangleContact>()
            .add_systems(OnEnter(SceneState::First), (systems::spawn_bloom_triangle,))
            .add_systems(Update, systems::handle_player_triangle_contact);
    }
}
