use crate::component::Component;

#[derive(Copy, Clone)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
      Self(0.0)
    }  
}
  

impl Component for Gravity {}
  