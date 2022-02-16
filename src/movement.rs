use ggez::{Context, input::keyboard, event::KeyCode, GameResult};

use crate::{
    system::System,
    component::{ComponentManager, Component},
    rigidbody::Rigidbody, vector2::Vector2,
};

#[derive(Copy, Clone)]
pub struct Movement {
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
          speed: 512.0
        }    
    }  
}

impl Component for Movement {}
  
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

                let mut x_axis = 0.0;
                let mut y_axis = 0.0;

                let pressed_left = keyboard::is_key_pressed(ctx, KeyCode::A) || keyboard::is_key_pressed(ctx, KeyCode::Left);
                let pressed_right = keyboard::is_key_pressed(ctx, KeyCode::D) || keyboard::is_key_pressed(ctx, KeyCode::Right);
                let pressed_up = keyboard::is_key_pressed(ctx, KeyCode::W) || keyboard::is_key_pressed(ctx, KeyCode::Up);
                let pressed_down = keyboard::is_key_pressed(ctx, KeyCode::S) || keyboard::is_key_pressed(ctx, KeyCode::Down);

                if pressed_left {
                    x_axis += -1.0;
                }
                if pressed_right {
                    x_axis += 1.0;
                }
                if pressed_up {
                    y_axis += -1.0;
                }
                if pressed_down {
                    y_axis += 1.0;
                }

                let delta_time = ggez::timer::delta(ctx).as_secs_f32();

                let x = x_axis * movement.speed * delta_time;
                let y = y_axis * movement.speed * delta_time;

                rigidbody.vel.smooth_damp(rigidbody.vel, Vector2::new(x, y), 0.03, delta_time);
            }
        }
    }

    fn draw(&self, _: &mut Context, _: usize, _: &ComponentManager) -> GameResult {
        Ok(())
    }
}