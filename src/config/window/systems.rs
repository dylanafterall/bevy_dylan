use super::resources::*;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::*,
};

// -----------------------------------------------------------------------------
pub fn setup_window(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = window_query.single_mut();

    let res = resolution._2560_x_1440;
    window.resolution.set(res.x, res.y);

    WindowResolution::set_scale_factor(&mut window.resolution, 1.0);
}

pub fn toggle_resolution(
    mut key_event: EventReader<KeyboardInput>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    for key in key_event.iter() {
        match key.state {
            ButtonState::Pressed => {
                let _key_option = match key.key_code {
                    Some(key_code) => {
                        match key_code {
                            KeyCode::Key1 => {
                                let res = resolution._1280_x_720;
                                window.resolution.set(res.x, res.y);
                            }
                            KeyCode::Key2 => {
                                let res = resolution._1920_x_1080;
                                window.resolution.set(res.x, res.y);
                            }
                            KeyCode::Key3 => {
                                let res = resolution._2560_x_1440;
                                window.resolution.set(res.x, res.y);
                            }
                            KeyCode::Key4 => {
                                let res = resolution._3840_x_2160;
                                window.resolution.set(res.x, res.y);
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
