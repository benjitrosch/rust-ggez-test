use crate::component::Component;

#[derive(Copy, Clone)]
pub struct Gravity(pub f64);

impl Component for Gravity {
    fn print(&self) {
        println!(
            "grav: {}",
            self.0
        );
    }
}
  