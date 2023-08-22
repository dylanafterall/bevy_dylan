mod components;
mod events;
mod resources;
mod systems;
mod game;
mod ui;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::*,
};

fn main() {
    App::new()
        // resources
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(resources::ResolutionSettings {
            _large: Vec2::new(1920.0, 1080.0),
            _medium: Vec2::new(800.0, 600.0),
            _small: Vec2::new(640.0, 360.0),
        })
        // plugins
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::Windowed,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: (640.0, 480.0).into(),
                    title: "".into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        // events
        // ...
        // systems: startup
        .add_systems(Startup, (
            systems::spawn_camera,
        ))
        // systems: update
        .add_systems(Update, (
            systems::toggle_resolution,
            systems::toggle_vsync,
            systems::toggle_cursor,
            systems::change_clear_color,
            systems::exit_game,
        ))
        // launch app
        .run();
}
