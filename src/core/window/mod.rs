mod systems;
mod resources;

use bevy::{
    prelude::*,
    window::*,
};
use crate::core::window::resources::ResolutionSettings;

// -----------------------------------------------------------------------------
pub struct CoreWindowPlugin;

impl Plugin for CoreWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::GRAY))
            .init_resource::<ResolutionSettings>()

            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::Windowed,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    title: "".into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))

            .add_systems(PreStartup, (
                systems::setup_window,
            ))
            .add_systems(Update, (
                systems::toggle_resolution,
                systems::toggle_vsync,
                systems::toggle_cursor,
                systems::change_clear_color,
            ));
    }
}