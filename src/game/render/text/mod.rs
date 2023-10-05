mod components;
mod systems;

use crate::game::scene_manager::SceneState;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct TextRenderPlugin;

impl Plugin for TextRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_text,))
            .add_systems(Update, (systems::translate_text, systems::rotate_text));
    }
}
