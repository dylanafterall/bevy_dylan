mod systems;

mod core;
mod game;

use crate::core::CorePlugin;
use crate::game::GamePlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        // plugins
        .add_plugins((
            CorePlugin,
            GamePlugin,
        ))
        // states
        .add_state::<AppState>()
        // systems: update
        .add_systems(Update, (
            systems::exit_game,
        ))
        // launch app
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}