use super::components::*;
use super::events::*;

use crate::config::window::{events::*, resources::AspectRatio};
use crate::style::FRAPPE_BASE;
use ::bevy::{
    core_pipeline::{
        bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping,
    },
    prelude::{Projection::*, *},
    render::{camera::ScalingMode::*, view::visibility::RenderLayers},
};
use crate::game::characters::player::components::Player;

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
            projection: Perspective(PerspectiveProjection { ..default() }),
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
            projection: Perspective(PerspectiveProjection { ..default() }),
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
            projection: Perspective(PerspectiveProjection { ..default() }),
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
pub fn handle_resolution_change(
    mut orthographic_query: Query<&mut OrthographicProjection>,
    mut perspective_query: Query<&mut Transform, With<Projection>>,
    mut res_change_events: EventReader<ChangeResolution>,
) {
    for res_change in res_change_events.iter() {
        //  - change the Fixed Width and Height for orthographic projections
        //  - change the z position of cameras with perspective projections
        let _desired_aspect_ratio = match &res_change.aspect_ratio {
            AspectRatio::_4_3 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = Fixed {
                        width: 256.0,
                        height: 192.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 231.765);
                }
            }
            AspectRatio::_5_4 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = Fixed {
                        width: 240.0,
                        height: 192.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 231.765);
                }
            }
            AspectRatio::_8_5 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = Fixed {
                        width: 256.0,
                        height: 160.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 193.137);
                }
            }
            AspectRatio::_16_9 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = Fixed {
                        width: 256.0,
                        height: 144.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 173.823);
                }
            }
            AspectRatio::_21_9 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = Fixed {
                        width: 336.0,
                        height: 144.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 173.823);
                }
            }
            AspectRatio::_32_9 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = Fixed {
                        width: 512.0,
                        height: 144.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 173.823);
                }
            }
        };
    }
}

pub fn emit_camera_zoom_in(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut zoom_in_event: EventWriter<CameraZoomIn>,
) {
    if keyboard_input.pressed(KeyCode::Equals) {
        zoom_in_event.send(CameraZoomIn);
        keyboard_input.reset(KeyCode::Equals);
    }
}

pub fn emit_camera_zoom_out(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut zoom_out_event: EventWriter<CameraZoomOut>,
) {
    if keyboard_input.pressed(KeyCode::Minus) {
        zoom_out_event.send(CameraZoomOut);
        keyboard_input.reset(KeyCode::Minus);
    }
}

pub fn emit_camera_move(
    player_query: Query<&Transform, With<Player>>,
    mut camera_move_event: EventWriter<CameraMove>,
) {
    let &player_transform = player_query.single();
    let player_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );

    camera_move_event.send(CameraMove {
        position: player_pos,
    });
}

pub fn handle_camera_move(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut event_listener: EventReader<CameraMove>,
) {
    for camera_move in event_listener.iter() {
        for mut camera in camera_query.iter_mut() {
            camera.translation = Vec3::new(
                camera_move.position.x,
                camera_move.position.y,
                camera.translation.z,   // don't change the current depth (z) value
            );
        }
    }
}

pub fn handle_camera_zoom_in(
    mut orthographic_query: Query<&mut OrthographicProjection, Without<UICamera>>,
    mut perspective_query: Query<(&mut Projection, &Transform)>,
    mut event_listener: EventReader<CameraZoomIn>,
) {
    for _ in event_listener.iter() {
        // will use aspect ratio and scale of 2d cameras to calculate new FOV for 3d cameras
        let mut aspect_ratio_y = 0.0;
        let mut new_scale = 0.0;

        for mut camera2d in orthographic_query.iter_mut() {
            camera2d.scale += 0.25;
            camera2d.scale = camera2d.scale.clamp(0.25, 5.0);

            new_scale = camera2d.scale;
            match camera2d.scaling_mode {
                Fixed { height, .. } => {
                    aspect_ratio_y = height;
                }
                _ => {}
            };
        }

        // top = y coordinate of perspective projection frustum top as it intersects z=0 plane (stage)
        let new_top = aspect_ratio_y * 0.5 * new_scale;

        for (projection3d, transform3d) in perspective_query.iter_mut() {
            let camera_z_pos = &transform3d.translation.z;
            let new_fov = 2.0 * (&new_top / camera_z_pos).atan();

            match projection3d.into_inner() {
                Perspective(proj) => {
                    proj.fov = new_fov;
                    // proj.fov is actually clamped/limited by 2d camera scale, since this calculates fov
                    proj.fov = proj.fov.clamp(10.0f32.to_radians(), 130.0f32.to_radians());
                }
                _ => {}
            };
        }
    }
}

pub fn handle_camera_zoom_out(
    mut orthographic_query: Query<&mut OrthographicProjection, Without<UICamera>>,
    mut perspective_query: Query<(&mut Projection, &Transform)>,
    mut event_listener: EventReader<CameraZoomOut>,
) {
    for _ in event_listener.iter() {
        // will use aspect ratio and scale of 2d cameras to calculate new FOV for 3d cameras
        let mut aspect_ratio_y = 0.0;
        let mut new_scale = 0.0;

        for mut camera2d in orthographic_query.iter_mut() {
            camera2d.scale -= 0.25;
            camera2d.scale = camera2d.scale.clamp(0.25, 5.0);

            new_scale = camera2d.scale;
            match camera2d.scaling_mode {
                Fixed { height, .. } => {
                    aspect_ratio_y = height;
                }
                _ => {}
            };
        }

        // top = y coordinate of perspective projection frustum top as it intersects z=0 plane (stage)
        let new_top = aspect_ratio_y * 0.5 * new_scale;

        for (projection3d, transform3d) in perspective_query.iter_mut() {
            let camera_z_pos = &transform3d.translation.z;
            let new_fov = 2.0 * (&new_top / camera_z_pos).atan();

            match projection3d.into_inner() {
                Perspective(proj) => {
                    proj.fov = new_fov;
                    // proj.fov is actually clamped/limited by 2d camera scale, since this calculates fov
                    proj.fov = proj.fov.clamp(10.0f32.to_radians(), 130.0f32.to_radians());
                }
                _ => {}
            };
        }
    }
}
