use crate::AppState;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct TransitionAppState {
    pub desired_app_state: AppState,
}
