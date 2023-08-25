mod settings_menu;
mod splash_screen;
mod title_screen;

use crate::ui::settings_menu::SettingsPlugin;
use crate::ui::splash_screen::SplashPlugin;
use crate::ui::title_screen::TitlePlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                SettingsPlugin,
                SplashPlugin,
                TitlePlugin,
            ));
    }
}