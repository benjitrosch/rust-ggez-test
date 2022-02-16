use std::ops;

#[derive(Copy, Clone)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32
}

#[allow(dead_code)]
impl Vector2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  pub fn lerp(&mut self, a: Vector2, b: Vector2, t: f32) -> Self {
    let x = a.x * (1.0 - t) + b.x * t;
    let y = a.y * (1.0 - t) + b.y * t;

    let s = Self {
      x,
      y
    };

    self.x = s.x;
    self.y = s.y;
    s
  }

  pub fn smooth_damp(&mut self, a: Vector2, b: Vector2, t: f32, dt: f32) -> Self {
    let smooth_time = if t > 0.0001 { t } else { 0.0001 };
    let omega = 2.0 / t;

    let dot = omega * dt;
    let exp = 1.0 / (1.0 + dot + 0.48 * dot * dot + 0.235 * dot * dot * dot);

    let mut change_x = a.x - b.x;
    let mut change_y = a.y - b.y;

    let mut current = a;
    let mut target = b;

    let max_change = std::f32::INFINITY * smooth_time;
    let max_change_sq =  max_change * max_change;

    let sq_dist = change_x * change_x + change_y * change_y;
    if sq_dist > max_change_sq {
      let mag = sq_dist.sqrt();
      change_x = change_x / mag * max_change;
      change_y = change_y / mag * max_change;
    }

    target.x = a.x - change_x;
    target.y = a.y - change_y;

    let temp_x = (a.x + omega * change_x) * dt;
    let temp_y = (a.y + omega * change_y) * dt;

    current.x = (a.x - omega * temp_x) * exp;
    current.y = (a.y - omega * temp_y) * exp;

    let mut output_x = target.x + (change_x + temp_x) * exp;
    let mut output_y = target.y + (change_y + temp_y) * exp;

    let orig_minus_current_x = a.x - current.x;
    let orig_minus_current_y = a.y - current.y;
    let out_minus_orig_x = output_x - a.x;
    let out_minus_orig_y = output_y - a.y;
    if orig_minus_current_x * out_minus_orig_x + orig_minus_current_y * out_minus_orig_y > 0.0
    {
      output_x = a.x;
      output_y = a.y;
    }

    let x = output_x;
    let y = output_y;

    let s = Self {
      x,
      y
    };

    self.x = s.x;
    self.y = s.y;
    s
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

impl ops::Mul<f32> for Vector2 {
  type Output = Self;

  fn mul(self, b: f32) -> Self::Output {
    Self {
      x: self.x * b,
      y: self.y * b
    }
  }
}

impl ops::Mul<Vector2> for f32 {
  type Output = Vector2;

  fn mul(self, b: Vector2) -> Self::Output {
    Vector2 {
      x: b.x * self,
      y: b.y * self
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
