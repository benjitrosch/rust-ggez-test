use crate::vector2::Vector2;
use crate::component::Component;

#[derive(Copy, Clone)]
pub struct Rigidbody {
    pub vel: Vector2,
    pub accel: Vector2
}

impl Default for Rigidbody {
    fn default() -> Self {
      Self {
        vel: Vector2::default(),
        accel: Vector2::default()
      }    
    }  
  }

impl Component for Rigidbody {}
  