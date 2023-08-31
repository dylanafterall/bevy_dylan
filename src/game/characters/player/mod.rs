pub mod components;
mod events;
mod systems;

use crate::AppState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_event::<events::PlayerMoveUp>()
            .add_event::<events::PlayerMoveDown>()
            .add_event::<events::PlayerMoveLeft>()
            .add_event::<events::PlayerMoveRight>()

            .add_systems(Update,(
                systems::emit_player_move_up,
                systems::emit_player_move_down,
                systems::emit_player_move_left,
                systems::emit_player_move_right,
                systems::handle_player_move_up,
                systems::handle_player_move_down,
                systems::handle_player_move_left,
                systems::handle_player_move_right,
                systems::handle_player_character_collision,
            ))

            .add_systems(OnEnter(AppState::Game), (
                systems::spawn_player,
            ));
    }
}