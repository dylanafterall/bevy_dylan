use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct PlayerMoveUp;

#[derive(Event)]
pub struct PlayerMoveDown;

#[derive(Event)]
pub struct PlayerMoveLeft;

#[derive(Event)]
pub struct PlayerMoveRight;
