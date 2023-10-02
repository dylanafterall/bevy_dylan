use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct Carrier {
    pub is_carrying: bool,
}

#[derive(Component)]
pub struct Carryable;
