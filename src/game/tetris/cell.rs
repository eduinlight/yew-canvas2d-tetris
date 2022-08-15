use crate::{game::EntityType, utils::plane::Point2D};
use derive_builder::Builder;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, Builder)]
pub struct Cell {
  pub position: Point2D<f32>,
  pub bg_color: String,
  pub border_color: String,
  pub size: i32,
  pub empty: bool,
}

impl EntityType for Cell {
  fn render(&self, context: std::rc::Rc<web_sys::CanvasRenderingContext2d>) {
    context.begin_path();
    context.set_fill_style(&JsValue::from_str(self.bg_color.as_str()));
    context.set_stroke_style(&JsValue::from_str(self.border_color.as_str()));
    context.rect(
      self.position.x as f64,
      self.position.y as f64,
      self.size as f64,
      self.size as f64,
    );
    context.fill();
    context.stroke();
  }
}
