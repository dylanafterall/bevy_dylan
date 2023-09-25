use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct CameraMove {
    pub position: Vec2,
}

#[derive(Event)]
pub struct CameraZoomIn;

#[derive(Event)]
pub struct CameraZoomOut;
