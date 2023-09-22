mod arc;
mod circle;
mod line;
mod ray;
mod rect;
mod systems;

use arc::ArcGizmoPlugin;
use circle::CircleGizmoPlugin;
use line::LineGizmoPlugin;
use ray::RayGizmoPlugin;
use rect::RectGizmoPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ArcGizmoPlugin,
            CircleGizmoPlugin,
            LineGizmoPlugin,
            RayGizmoPlugin,
            RectGizmoPlugin,
        ))
        .add_systems(Startup, systems::setup_gizmos);
    }
}
