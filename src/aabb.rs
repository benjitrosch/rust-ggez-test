use ggez::{Context, GameResult, graphics::{self, Color}, mint::Vector2 as Vector2f};

use crate::{
    system::System,
    component::{ComponentManager, Component},
    transform::Transform, rigidbody::Rigidbody, vector2::Vector2
};


#[derive(Copy, Clone)]
pub struct AABB {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

#[allow(dead_code)]
impl AABB {
    pub fn check(&self, b: AABB) -> bool {
        self.x < b.x + b.w &&
        self.x + self.w  > b.x &&
        self.y < b.y + b.h &&
        self.y + self.h > b.y
    }

    pub fn translate(&self, dir: Vector2) -> AABB {
        Self {
            x: self.x + dir.x,
            y: self.y + dir.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn at(&self, pos: Vector2) -> AABB {
        Self {
            x: pos.x,
            y: pos.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl Default for AABB {
  fn default() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      w: 32.0,
      h: 32.0,
    }    
  }  
}

impl Component for AABB {}

pub struct CollisionSystem {}

impl Default for CollisionSystem {
    fn default() -> Self {
        Self {}    
    }  
}

impl System for CollisionSystem {
    fn update(&mut self, _: &mut Context, entities: usize, component_manager: &mut ComponentManager) {
        for entity in 0..entities {
            let has_transform_component = component_manager.entity_has_component::<Transform>(entity);
            let has_rigidbody_component = component_manager.entity_has_component::<Rigidbody>(entity);
            let has_aabb_component = component_manager.entity_has_component::<AABB>(entity);

            if has_transform_component && has_aabb_component {
                let transform_list = component_manager.get_components::<Transform>().unwrap();
                let transform = *transform_list.get_entity_component(entity).unwrap();

                let aabb_list_mut = component_manager.get_components_mut::<AABB>().unwrap();
                let aabb_ref = aabb_list_mut.get_entity_component_mut(entity).unwrap();

                aabb_ref.x = transform.pos.x;
                aabb_ref.y = transform.pos.y;
                let aabb = *aabb_ref;

                if has_rigidbody_component {
                    let rigidbody_list = component_manager.get_components::<Rigidbody>().unwrap();
                    let rigidbody = *rigidbody_list.get_entity_component(entity).unwrap();
                    let vel = rigidbody.vel;

                    let new_position_x = Vector2::new(
                        transform.pos.x + vel.x,
                        transform.pos.y
                    );
                    let new_position_y = Vector2::new(
                        transform.pos.x,
                        transform.pos.y + vel.y
                    );

                    let aabb_x = aabb.at(new_position_x);
                    let aabb_y = aabb.at(new_position_y);

                    let aabb_list = component_manager.get_components::<AABB>().unwrap().clone();

                    for e in 0..entities {
                        if e == entity {
                            continue;
                        }

                        let aabb_b = *aabb_list.get_entity_component(e).unwrap();

                        if aabb_x.check(aabb_b) {
                            let rigidbody_list_mut = component_manager.get_components_mut::<Rigidbody>().unwrap();
                            let rigidbody_ref = rigidbody_list_mut.get_entity_component_mut(entity).unwrap();
                            rigidbody_ref.vel.x = 0.0;
    
                            let transform_list_mut = component_manager.get_components_mut::<Transform>().unwrap();
                            let transform_ref = transform_list_mut.get_entity_component_mut(entity).unwrap();

                            if vel.x > 0.1 {
                                transform_ref.pos.x = aabb_b.x - aabb.w;
                            }
                            else if vel.x < 0.1 {
                                transform_ref.pos.x = aabb_b.x + aabb_b.w;
                            }
                        }

                        if aabb_y.check(aabb_b) {
                            let rigidbody_list_mut = component_manager.get_components_mut::<Rigidbody>().unwrap();
                            let rigidbody_ref = rigidbody_list_mut.get_entity_component_mut(entity).unwrap();
                            rigidbody_ref.vel.y = 0.0;
    
                            let transform_list_mut = component_manager.get_components_mut::<Transform>().unwrap();
                            let transform_ref = transform_list_mut.get_entity_component_mut(entity).unwrap();

                            if vel.y > 0.1 {
                                transform_ref.pos.y = aabb_b.y - aabb.h;
                            }
                            else if vel.y < 0.1 {
                                transform_ref.pos.y = aabb_b.y + aabb_b.h;
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, ctx: &mut Context, entities: usize, component_manager: &ComponentManager) -> GameResult {
        for entity in 0..entities {
            let has_aabb_component = component_manager.entity_has_component::<AABB>(entity);
            if has_aabb_component {
                let aabb = component_manager.get_components::<AABB>().unwrap().get_entity_component(entity).unwrap();

                let rect = graphics::Rect::new(
                    0.0,
                    0.0,
                    aabb.w,
                    aabb.h
                );
                let rect_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(3.2),
                    rect,
                    Color::RED
                )?;

                let pos = Vector2f {
                    x: aabb.x - aabb.w * 0.5,
                    y: aabb.y - aabb.h * 0.5
                };
                let draw_params = graphics::DrawParam::new()
                    .dest(pos);

                graphics::draw(ctx, &rect_mesh, draw_params)?;
            }
        }

        Ok(())
    }
}