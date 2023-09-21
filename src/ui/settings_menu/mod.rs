mod audio;
mod controller;
mod credits;
mod gameplay;
mod keyboard;
mod video;

use audio::AudioSettingsPlugin;
use bevy::prelude::*;
use controller::ControllerSettingsPlugin;
use credits::CreditsPlugin;
use gameplay::GameplaySettingsPlugin;
use keyboard::KeyboardSettingsPlugin;
use video::VideoSettingsPlugin;

// -----------------------------------------------------------------------------
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AudioSettingsPlugin,
            ControllerSettingsPlugin,
            CreditsPlugin,
            GameplaySettingsPlugin,
            KeyboardSettingsPlugin,
            VideoSettingsPlugin,
        ))
        .add_state::<SettingsState>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SettingsState {
    #[default]
    Inert,
    Gameplay,
    Video,
    Audio,
    Controller,
    Keyboard,
    Credits,
}
