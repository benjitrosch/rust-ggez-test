use ggez::{Context, input::keyboard, event::KeyCode};

use crate::{
    system::System,
    component::{ComponentManager, Component},
    rigidbody::Rigidbody,
};

#[derive(Copy, Clone)]
pub struct Movement {
    pub speed: f64,
}

impl Component for Movement {
    fn print(&self) {
        println!(
            "",
        );
    }
}
  

pub struct MovementSystem {}

impl Default for MovementSystem {
    fn default() -> Self {
        Self {}    
    }  
}

impl System for MovementSystem {
    fn update(&mut self, ctx: &mut Context, entities: usize, component_manager: &mut ComponentManager) {
        for entity in 0..entities {
            let has_rigidbody_component = component_manager.entity_has_component::<Rigidbody>(entity);
            let has_movement_component = component_manager.entity_has_component::<Movement>(entity);

            if has_rigidbody_component && has_movement_component {
                let movement_list = component_manager.get_components::<Movement>().unwrap();
                let movement = *movement_list.get_entity_component(entity).unwrap();
                
                let rigidbody_list = component_manager.get_components_mut::<Rigidbody>().unwrap();
                let rigidbody = rigidbody_list.get_entity_component_mut(entity).unwrap();

                let delta_time = ggez::timer::delta(ctx).as_secs_f64();

                rigidbody.vel.y = if keyboard::is_key_pressed(ctx, KeyCode::W) {
                    -movement.speed * delta_time
                }
                else if keyboard::is_key_pressed(ctx, KeyCode::S) {
                    movement.speed * delta_time
                }
                else {
                    0.0
                };

                rigidbody.vel.x = if keyboard::is_key_pressed(ctx, KeyCode::A) {
                    -movement.speed * delta_time
                }
                else if keyboard::is_key_pressed(ctx, KeyCode::D) {
                    movement.speed * delta_time
                }
                else {
                    0.0
                };
            }
        }
    }
}