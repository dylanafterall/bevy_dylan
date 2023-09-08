mod events;
mod systems;
mod system_params;

mod core;
mod game;
mod ui;

use crate::core::CorePlugin;
use crate::game::GamePlugin;
use crate::ui::UIPlugin;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// -----------------------------------------------------------------------------
fn main() {
    App::new()
        .add_plugins((
            CorePlugin,
            GamePlugin,
            UIPlugin,
            ShapePlugin,
            RapierPhysicsPlugin::<system_params::MyPhysicsHooks>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ))

        .add_state::<AppState>()

        .add_event::<events::TransitionToSplash>()
        .add_event::<events::TransitionToTitle>()
        .add_event::<events::TransitionToSettings>()
        .add_event::<events::TransitionToGame>()
        .add_event::<events::TransitionToFail>()

        .add_systems(Update, (
            systems::emit_transition_to_title,
            systems::emit_transition_to_game,
            systems::handle_transition_splash_to_title.run_if(in_state(AppState::Splash)),
            systems::handle_transition_settings_to_title.run_if(in_state(AppState::Settings)),
            systems::handle_transition_game_to_title.run_if(in_state(AppState::Game)),
            systems::handle_transition_title_to_settings.run_if(in_state(AppState::Title)),
            systems::handle_transition_title_to_game.run_if(in_state(AppState::Title)),
            systems::handle_transition_fail_to_game.run_if(in_state(AppState::Fail)),
            systems::handle_transition_game_to_fail.run_if(in_state(AppState::Game)),
            systems::exit_game,
        ))

        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Splash,
    #[default]
    Title,
    Settings,
    Game,
    Fail,
}