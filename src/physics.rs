use ggez::Context;

use crate::{
    system::System,
    vector2::Vector2,
    component::ComponentManager,
    transform::Transform,
    rigidbody::Rigidbody,
    gravity::Gravity,
};

pub struct PhysicsSystem {}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self {}    
    }  
}

impl System for PhysicsSystem {
    fn update(&mut self, _: &mut Context, entities: usize, component_manager: &mut ComponentManager) {
        for entity in 0..entities {
            let has_transform_component = component_manager.entity_has_component::<Transform>(entity);
            let has_rigidbody_component = component_manager.entity_has_component::<Rigidbody>(entity);
            let has_gravity_component = component_manager.entity_has_component::<Gravity>(entity);

            if has_transform_component && has_rigidbody_component && has_gravity_component {
                let transform_list = component_manager.get_components::<Transform>().unwrap();
                let transform = *transform_list.get_entity_component(entity).unwrap();

                let rigidbody_list = component_manager.get_components::<Rigidbody>().unwrap();
                let rigidbody = *rigidbody_list.get_entity_component(entity).unwrap();

                let gravity_list = component_manager.get_components::<Gravity>().unwrap();
                let gravity = *gravity_list.get_entity_component(entity).unwrap();

                let new_velocity = Vector2::new(
                    rigidbody.vel.x,
                    rigidbody.vel.y - gravity.0,
                );

                let new_position = Vector2::new(
                    transform.pos.x + rigidbody.vel.x,
                    transform.pos.y + rigidbody.vel.y
                );

                let rigidbody_list_mut = component_manager.get_components_mut::<Rigidbody>().unwrap();
                let rigidbody_ref = rigidbody_list_mut.get_entity_component_mut(entity).unwrap();
                rigidbody_ref.vel = new_velocity;

                let transform_list_mut = component_manager.get_components_mut::<Transform>().unwrap();
                let transform_ref = transform_list_mut.get_entity_component_mut(entity).unwrap();
                transform_ref.pos = new_position;
            }
        }
    }
}