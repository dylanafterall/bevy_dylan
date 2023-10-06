use super::components::*;
use crate::style::FRAPPE_TEXT;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/OpenSans/OpenSans-Medium.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 200.0,
        color: FRAPPE_TEXT,
    };
    let text_alignment = TextAlignment::Center;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translating text", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        TextTranslation,
    ));

    // Demonstrate changing rotation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("rotating text", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        TextRotation,
    ));
}

pub fn translate_text(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<TextTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 10.0 * time.elapsed_seconds().sin() - 70.0;
        transform.translation.y = 10.0 * time.elapsed_seconds().cos() + 35.0;
        transform.scale = Vec3::splat(0.05);
    }
}

pub fn rotate_text(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<TextRotation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 70.0;
        transform.translation.y = 35.0;
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
        transform.scale = Vec3::splat(0.05);
    }
}
