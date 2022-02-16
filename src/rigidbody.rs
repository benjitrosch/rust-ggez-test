use crate::vector2::Vector2;
use crate::component::Component;

#[derive(Copy, Clone)]
pub struct Rigidbody {
    pub vel: Vector2
}

impl Default for Rigidbody {
    fn default() -> Self {
      Self {
        vel: Vector2::new(2.0, 1.0)
      }    
    }  
  }

impl Component for Rigidbody {}
  