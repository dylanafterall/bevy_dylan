mod events;
mod systems;

mod core;
mod game;

use crate::core::CorePlugin;
use crate::game::GamePlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
fn main() {
    App::new()
        // plugins
        .add_plugins((
            CorePlugin,
            GamePlugin,
        ))
        // states
        .add_state::<AppState>()
        // events
        .add_event::<events::TransitionToSplash>()
        .add_event::<events::TransitionToTitle>()
        .add_event::<events::TransitionToSettings>()
        .add_event::<events::TransitionToGame>()
        .add_event::<events::TransitionToFail>()
        // systems: update
        .add_systems(Update, (
            systems::emit_transition_to_title,
            systems::emit_transition_to_settings,
            systems::emit_transition_to_game,
            systems::emit_transition_to_fail,
            systems::handle_transition_splash_to_title.run_if(in_state(AppState::Splash)),
            systems::handle_transition_settings_to_title.run_if(in_state(AppState::Settings)),
            systems::handle_transition_game_to_title.run_if(in_state(AppState::Game)),
            systems::handle_transition_title_to_settings.run_if(in_state(AppState::Title)),
            systems::handle_transition_title_to_game.run_if(in_state(AppState::Title)),
            systems::handle_transition_fail_to_game.run_if(in_state(AppState::Fail)),
            systems::handle_transition_game_to_fail.run_if(in_state(AppState::Game)),
            systems::exit_game,
        ))
        // launch app
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    Title,
    Settings,
    Game,
    Fail,
}