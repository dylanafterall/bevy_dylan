use bevy::{
    app::*,
    prelude::*,
    window::*,
};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<crate::resources::ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution._small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution._medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let res = resolution._large;
        window.resolution.set(res.x, res.y);
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
    if input.just_pressed(KeyCode::Space) {
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}