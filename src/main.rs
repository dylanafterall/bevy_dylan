mod events;
mod system_params;
mod systems;

mod config;
mod game;
pub mod materials;
mod style;
mod ui;

use crate::config::ConfigPlugin;
use crate::game::GamePlugin;
use crate::materials::MaterialPluginGroup;
use crate::ui::UIPlugin;

use bevy::{
    prelude::*,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::*,
};
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;
use std::error::Error;

// -----------------------------------------------------------------------------
const GRAV_MAG: f32 = 40.0; // used with RapierConfiguration resource

fn main() -> Result<(), Box<dyn Error>> {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::new()
        .add_plugins((
            // bevy plugins
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        title: "".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::from(wgpu_settings),
                }),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
            // my plugins
            ConfigPlugin,
            GamePlugin,
            UIPlugin,
            MaterialPluginGroup,
            // external crate plugins
            RapierPhysicsPlugin::<system_params::MyPhysicsHooks>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin {
                enabled: true,
                style: DebugRenderStyle {
                    rigid_body_axes_length: 2.0,
                    ..default()
                },
                mode: Default::default(),
            },
            WorldInspectorPlugin::new(),
            HanabiPlugin,
            ShapePlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -GRAV_MAG),
            ..Default::default()
        })
        .add_state::<AppState>()
        .add_event::<events::TransitionAppState>()
        .add_systems(
            Update,
            (
                systems::emit_transition_to_title,
                systems::emit_transition_to_game,
                systems::handle_transition_app_state,
                systems::exit_game,
            ),
        )
        .run();

    Ok(())
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Splash,
    #[default]
    Title,
    Settings,
    Game,
    Fail,
}
