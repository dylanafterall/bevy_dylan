use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::Vector;

// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct ConveyorBelt;

#[derive(Component)]
pub struct OneWayPlatform;

#[derive(SystemParam)]
pub struct MyPhysicsHooks<'w, 's> {
    belt_query: Query<'w, 's, &'static ConveyorBelt>,
    platform_query: Query<'w, 's, &'static OneWayPlatform>,
}

impl BevyPhysicsHooks for MyPhysicsHooks<'_, '_> {
    fn modify_solver_contacts(&self, context: ContactModificationContextView) {
        let entity1 = context.collider1();
        let entity2 = context.collider2();

        // conveyor belt functionality -----------------------------------------
        if self.belt_query.get(entity1).is_ok() {
            for solver_contact in &mut *context.raw.solver_contacts {
                solver_contact.tangent_velocity.x = 50.0;
            }
        } else if self.belt_query.get(entity2).is_ok() {
            for solver_contact in &mut *context.raw.solver_contacts {
                solver_contact.tangent_velocity.x = -50.0;
            }
        }

        // one-way platform functionality --------------------------------------
        let mut one_way_exists = false;
        let mut allowed_local_n1 = Vector::zeros();

        if self.platform_query.get(entity1).is_ok() {
            println!("entity 1");
            allowed_local_n1 = Vector::y();
            one_way_exists = true;
        }
        if self.platform_query.get(entity2).is_ok() {
            println!("entity 2");
            allowed_local_n1 = -Vector::y();
            one_way_exists = true;
        }

        if one_way_exists {
            context
                .raw
                .update_as_oneway_platform(&allowed_local_n1, 0.1);
        }
    }
}
