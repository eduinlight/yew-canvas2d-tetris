#[derive(Debug, Clone)]
pub struct Size {
  pub width: f64,
  pub height: f64,
}

pub trait JsAnimationFrame {
  fn frame(&self, elapsed: f32);
}
