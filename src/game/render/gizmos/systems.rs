use bevy::{prelude::*, render::view::RenderLayers};

// -----------------------------------------------------------------------------
pub fn setup_gizmos(mut config: ResMut<GizmoConfig>) {
    config.enabled = true;
    config.line_width = 3.5;
    config.line_perspective = false;
    config.depth_bias = -0.1;
    config.render_layers = RenderLayers::layer(0);
}
