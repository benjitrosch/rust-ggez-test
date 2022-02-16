use crate::vector2::Vector2;
use crate::component::Component;

#[derive(Copy, Clone)]
pub struct Transform {
  pub pos: Vector2
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      pos: Vector2::new(0.0, 0.0)
    }    
  }  
}

impl Component for Transform {}
