mod systems;
mod resources;

use bevy::{
    prelude::*,
    window::*,
};

// -----------------------------------------------------------------------------
pub struct CoreWindowPlugin;

impl Plugin for CoreWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::GRAY))
            .insert_resource(resources::ResolutionSettings {
                _800_x_1280: Vec2::new(800.0, 1280.0),
                _1024_x_768: Vec2::new(1024.0, 768.0),
                _1280_x_1024: Vec2::new(1280.0, 1024.0),
                _1280_x_720: Vec2::new(1280.0, 720.0),
                _1280_x_800: Vec2::new(1280.0, 800.0),
                _1360_x_768: Vec2::new(1360.0, 768.0),
                _1366_x_768: Vec2::new(1366.0, 768.0),
                _1440_x_900: Vec2::new(1440.0, 900.0),
                _1600_x_900: Vec2::new(1600.0, 900.0),
                _1680_x_1050: Vec2::new(1680.0, 1050.0),
                _1920_x_1200: Vec2::new(1920.0, 1200.0),
                _1920_x_1080: Vec2::new(1920.0, 1080.0),
                _2560_x_1440: Vec2::new(2560.0, 1440.0),
                _2560_x_1600: Vec2::new(2560.0, 1600.0),
                _2560_x_1080: Vec2::new(2560.0, 1080.0),
                _2880_x_1800: Vec2::new(2880.0, 1800.0),
                _3440_x_1440: Vec2::new(3440.0, 1440.0),
                _3840_x_2160: Vec2::new(3840.0, 2160.0),
                _5120_x_1440: Vec2::new(5120.0, 1440.0),
            })
            // plugins
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