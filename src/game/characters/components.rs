use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Component)]
pub enum NPC {
    Friendly,
    Neutral,
    Hostile,
}
