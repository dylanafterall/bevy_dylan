pub mod components;

mod friendly;
mod hostile;
pub mod player;

use crate::game::characters::friendly::FriendlyPlugin;
use crate::game::characters::hostile::HostilePlugin;
use crate::game::characters::player::PlayerPlugin;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FriendlyPlugin, HostilePlugin, PlayerPlugin));
    }
}
