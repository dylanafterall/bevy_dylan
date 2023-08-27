use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct TransitionToFirstLevel {}

#[derive(Event)]
pub struct TransitionToSecondLevel {}

#[derive(Event)]
pub struct TransitionToThirdLevel {}