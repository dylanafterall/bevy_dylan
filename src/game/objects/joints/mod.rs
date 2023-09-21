pub mod components;

mod fan;
mod fixed_joint;
mod multi_collider;
mod prismatic_joint;
mod revolute_joint;

use fan::FanPlugin;
use fixed_joint::FixedJointPlugin;
use multi_collider::MultiColliderPlugin;
use prismatic_joint::PrismaticJointPlugin;
use revolute_joint::RevoluteJointPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct JointsPlugin;

impl Plugin for JointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FanPlugin,
            FixedJointPlugin,
            MultiColliderPlugin,
            PrismaticJointPlugin,
            RevoluteJointPlugin,
        ));
    }
}
