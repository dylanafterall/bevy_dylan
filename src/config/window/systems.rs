use super::resources::*;

use super::events::ChangeResolution;
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::*,
};

// -----------------------------------------------------------------------------
pub fn setup_window(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    res_settings: Res<ResolutionSettings>,
) {
    let mut window = window_query.single_mut();

    // set default resolution
    let res = res_settings._2560_1440;
    window.resolution.set(res.x, res.y);

    WindowResolution::set_scale_factor_override(&mut window.resolution, Some(1.0));
}

pub fn emit_resolution_change(
    mut key_event: EventReader<KeyboardInput>,
    res_settings: Res<ResolutionSettings>,
    mut res_change_event: EventWriter<ChangeResolution>,
) {
    for key in key_event.read() {
        match key.state {
            ButtonState::Pressed => {
                let _key_option = match key.key_code {
                    Some(key_code) => {
                        match key_code {
                            KeyCode::Key1 => {
                                res_change_event.send(ChangeResolution {
                                    resolution: res_settings._1024_768,
                                    aspect_ratio: AspectRatio::_4_3,
                                });
                            }
                            KeyCode::Key2 => {
                                res_change_event.send(ChangeResolution {
                                    resolution: res_settings._1280_1024,
                                    aspect_ratio: AspectRatio::_5_4,
                                });
                            }
                            KeyCode::Key3 => {
                                res_change_event.send(ChangeResolution {
                                    resolution: res_settings._2560_1600,
                                    aspect_ratio: AspectRatio::_8_5,
                                });
                            }
                            KeyCode::Key4 => {
                                res_change_event.send(ChangeResolution {
                                    resolution: res_settings._1920_1080,
                                    aspect_ratio: AspectRatio::_16_9,
                                });
                            }
                            KeyCode::Key5 => {
                                res_change_event.send(ChangeResolution {
                                    resolution: res_settings._2560_1440,
                                    aspect_ratio: AspectRatio::_16_9,
                                });
                            }
                            _ => {}
                        };
                    }
                    None => {}
                };
            }
            ButtonState::Released => {}
        }
    }
}

pub fn handle_resolution_change(
    mut windows: Query<&mut Window>,
    mut res_change_events: EventReader<ChangeResolution>,
) {
    let mut window = windows.single_mut();

    for res_change in res_change_events.read() {
        let desired_res = res_change.resolution;
        window.resolution.set(desired_res.x, desired_res.y);
    }
}

pub fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::V) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}

pub fn toggle_cursor(mut windows: Query<&mut Window>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Key0) {
        let mut window = windows.single_mut();

        window.cursor.visible = !window.cursor.visible;
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}

pub fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::C) {
        clear_color.0 = Color::DARK_GRAY;
    }
}
