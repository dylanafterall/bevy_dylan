use super::components::*;
use super::events::*;

use::bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    render::view::visibility::RenderLayers,
};

// -----------------------------------------------------------------------------
pub fn spawn_world_and_ui_cameras(mut commands: Commands) {
    // World (default) camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        UiCameraConfig {
            show_ui: false,
        },
        WorldCamera,
        RenderLayers::from_layers(&[0]),
    ));

    // UI camera
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        UICamera,
        RenderLayers::from_layers(&[2]),
    ));
}

pub fn spawn_player_camera(mut commands: Commands) {
    // Player camera
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        UiCameraConfig {
            show_ui: false,
        },
        PlayerCamera,
        RenderLayers::from_layers(&[1]),
    ));
}

pub fn despawn_player_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<PlayerCamera>>,
) {
    for camera in camera_query.iter() {
        commands.entity(camera).despawn();
    }
}

pub fn handle_camera_zoom_in(
    mut query: Query<&mut OrthographicProjection, Without<UICamera>>,
    mut event_listener: EventReader<CameraZoomIn>,
) {
    for _ in event_listener.iter() {
        for mut camera in query.iter_mut() {
            camera.scale *= 1.25;
            camera.scale = camera.scale.clamp(0.5, 5.0);
        }
    }
}

pub fn handle_camera_zoom_out(
    mut query: Query<&mut OrthographicProjection, Without<UICamera>>,
    mut event_listener: EventReader<CameraZoomOut>,
) {
    for _ in event_listener.iter() {
        for mut camera in query.iter_mut() {
            camera.scale *= 0.75;
            camera.scale = camera.scale.clamp(0.5, 5.0);
        }
    }
}