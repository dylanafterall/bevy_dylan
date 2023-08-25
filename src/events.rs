use bevy::prelude::*;

#[derive(Event)]
pub struct TransitionToSplash {}

#[derive(Event)]
pub struct TransitionToTitle {}

#[derive(Event)]
pub struct TransitionToSettings {}

#[derive(Event)]
pub struct TransitionToGame {}

#[derive(Event)]
pub struct TransitionToFail {}