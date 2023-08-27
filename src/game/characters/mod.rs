mod friendly;
mod hostile;
mod player;

use bevy::prelude::*;
use crate::game::characters::friendly::FriendlyPlugin;
use crate::game::characters::hostile::HostilePlugin;
use crate::game::characters::player::PlayerPlugin;

// -----------------------------------------------------------------------------
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FriendlyPlugin,
                HostilePlugin,
                PlayerPlugin,
            ));
    }
}
