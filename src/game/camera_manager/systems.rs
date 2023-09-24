use super::components::*;
use super::events::*;

use crate::style::FRAPPE_BASE;
use ::bevy::{
    core_pipeline::{
        bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping,
    },
    prelude::*,
    render::{camera::ScalingMode::*, view::visibility::RenderLayers},
};

// -----------------------------------------------------------------------------
/*
       Changing resolution of window:
           for 2D cameras, adjust OrthographicProjection{scaling_mode: Fixed{width: and height:}}
           for 3D cameras, adjust transform: Transform::from_translation(_,_,z)
       Calculating z position for 3D camera:
           z = ( height / 2 ) / ( tan( FOV / 2 ) )
*/
pub fn spawn_cameras(mut commands: Commands) {
    //      ORDER       RENDER LAYER            CAMERA              CONFIG
    //      -----       ------------            ------              ------
    //        0             1               Background          3D Perspective
    //        1             0               Stage               2D Orthographic
    //        2             2               Particles           3D Perspective
    //        3             3               Player              2D Orthographic
    //        4             4               Foreground          3D Perspective
    //        5             5               UI                  2D Orthographic

    // BACKGROUND --------------------------------------------------------------
    commands.spawn((
        Name::new("CameraBackground"),
        BackgroundCamera,
        UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[1]),
        Camera3dBundle {
            camera: Camera {
                order: 0,
                // hdr: true,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(FRAPPE_BASE),
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection { ..default() }),
            tonemapping: Tonemapping::None,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 173.82)),
            ..default()
        },
        BloomSettings::default(),
    ));

    // STAGE -------------------------------------------------------------------
    commands.spawn((
        Name::new("CameraStage"),
        StageCamera,
        UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[0, 1]),
        Camera2dBundle {
            camera: Camera {
                order: 1,
                hdr: false,
                ..default()
            },
            projection: OrthographicProjection {
                near: -1.0,
                scaling_mode: Fixed {
                    width: 256.0,
                    height: 144.0,
                },
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings::default(),
    ));

    // PARTICLES ---------------------------------------------------------------
    commands.spawn((
        Name::new("CameraParticles"),
        ParticlesCamera,
        UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[2]),
        Camera3dBundle {
            camera: Camera {
                order: 2,
                // hdr: true,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection { ..default() }),
            tonemapping: Tonemapping::None,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 173.82)),
            ..default()
        },
        BloomSettings::default(),
    ));

    // PLAYER ------------------------------------------------------------------
    commands.spawn((
        Name::new("CameraPlayer"),
        PlayerCamera,
        UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[3]),
        Camera2dBundle {
            camera: Camera {
                order: 3,
                // hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                near: -1.0,
                scaling_mode: Fixed {
                    width: 256.0,
                    height: 144.0,
                },
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings::default(),
    ));

    // FOREGROUND --------------------------------------------------------------
    commands.spawn((
        Name::new("CameraForeground"),
        ForegroundCamera,
        UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[4]),
        Camera3dBundle {
            camera: Camera {
                order: 4,
                // hdr: true,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection { ..default() }),
            tonemapping: Tonemapping::None,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 173.82)),
            ..default()
        },
        BloomSettings::default(),
    ));

    // UI ----------------------------------------------------------------------
    commands.spawn((
        Name::new("CameraUI"),
        UICamera,
        RenderLayers::from_layers(&[5]),
        Camera2dBundle {
            camera: Camera {
                order: 5,
                // hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                near: -1.0,
                scaling_mode: Fixed {
                    width: 256.0,
                    height: 144.0,
                },
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            tonemapping: Tonemapping::None,
            ..default()
        },
        BloomSettings::default(),
    ));
}

// cinematography
// -----------------------------------------------------------------------------
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
