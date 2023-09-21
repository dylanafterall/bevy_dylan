use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct ElevatorTimer {
    pub timer: Timer,
    pub heading_up: bool,
}
