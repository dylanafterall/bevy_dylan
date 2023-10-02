pub mod carry;
pub mod components;
mod events;
mod systems;

use crate::game::scene_manager::SceneState;
use crate::game::GameState;
use carry::CarryPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CarryPlugin)
            .add_event::<events::PlayerMoveUp>()
            .add_event::<events::PlayerMoveDown>()
            .add_event::<events::PlayerMoveLeft>()
            .add_event::<events::PlayerMoveRight>()
            .add_systems(
                Update,
                (
                    systems::emit_player_move_up,
                    systems::emit_player_move_down,
                    systems::emit_player_move_left,
                    systems::emit_player_move_right,
                    systems::handle_player_move_up.run_if(in_state(GameState::Playing)),
                    systems::handle_player_move_down.run_if(in_state(GameState::Playing)),
                    systems::handle_player_move_left.run_if(in_state(GameState::Playing)),
                    systems::handle_player_move_right.run_if(in_state(GameState::Playing)),
                    systems::handle_player_character_collision,
                ),
            )
            .add_systems(OnEnter(SceneState::First), (systems::spawn_player,))
            .add_systems(OnEnter(SceneState::Second), (systems::spawn_player,))
            .add_systems(OnEnter(SceneState::Third), (systems::spawn_player,))
            .add_systems(OnEnter(SceneState::Fourth), (systems::spawn_player,))
            .add_systems(OnEnter(SceneState::Fifth), (systems::spawn_player,));
    }
}
