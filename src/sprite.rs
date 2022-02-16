use ggez::{graphics::{self, Color}, Context, mint::Vector2, GameResult};

use crate::{
    system::System,
    component::{ComponentManager, Component}, transform::Transform,
};

#[derive(Copy, Clone)]
pub struct Sprite {
    pub w: u8,
    pub h: u8,
    pub color: graphics::Color,
}

impl Default for Sprite {
    fn default() -> Self {
      Self {
        w: 32,
        h: 32,
        color: Color::WHITE,
      }    
    }  
}  

impl Component for Sprite {}
  
pub struct RenderSystem {}

impl Default for RenderSystem {
    fn default() -> Self {
        Self {}    
    }  
}

impl System for RenderSystem {
    fn update(&mut self, _: &mut Context, _: usize, _: &mut ComponentManager) {
    }

    fn draw(&self, ctx: &mut Context, entities: usize, component_manager: &ComponentManager) -> GameResult {
        for entity in 0..entities {
            let has_sprite_component = component_manager.entity_has_component::<Sprite>(entity);
            let has_transform_component = component_manager.entity_has_component::<Transform>(entity);

            if has_sprite_component && has_transform_component {
                let sprite = component_manager.get_components::<Sprite>().unwrap().get_entity_component(entity).unwrap();

                let rect = graphics::Rect::new(
                    0.0,
                    0.0,
                    sprite.w as f32,
                    sprite.h as f32
                );
                let rect_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    sprite.color
                )?;

                let player_position = component_manager.get_components::<Transform>().unwrap().get_entity_component(entity).unwrap().pos;
                let pos = Vector2 {
                    x: player_position.x - sprite.w as f32 * 0.5,
                    y: player_position.y - sprite.h as f32 * 0.5
                };
                let draw_params = graphics::DrawParam::new()
                    .dest(pos);

                graphics::draw(ctx, &rect_mesh, draw_params)?;
            }
        }

        Ok(())
    }
}