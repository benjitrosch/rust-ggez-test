use std::ops;

#[derive(Copy, Clone)]
pub struct Vector2 {
  pub x: f64,
  pub y: f64
}

impl Vector2 {
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }
}

impl Default for Vector2 {
  fn default() -> Self {
    Self {
      x: 0.0,
      y: 0.0
    }    
  }  
}

impl ops::Add<Vector2> for Vector2 {
  type Output = Self;

  fn add(self, b: Self) -> Self::Output {
    Self {
      x: self.x + b.x,
      y: self.y + b.y
    }
  }
}

impl ops::Sub<Vector2> for Vector2 {
  type Output = Self;

  fn sub(self, b: Self) -> Self::Output {
    Self {
      x: self.x + b.x,
      y: self.y + b.y
    }
  }
}

impl ops::Mul<Vector2> for Vector2 {
  type Output = Self;

  fn mul(self, b: Self) -> Self::Output {
    Self {
      x: self.x * b.x,
      y: self.y * b.y
    }
  }
}

impl ops::Div<Vector2> for Vector2 {
  type Output = Self;

  fn div(self, b: Self) -> Self::Output {
    Self {
      x: self.x / b.x,
      y: self.y / b.y
    }
  }
}
